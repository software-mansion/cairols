#!/bin/bash
#
# Reports pending PR review requests across the Cairo / Scarb tooling repos.
#
# For every repo in $REPOS, queries the GitHub GraphQL API for open PRs, groups
# outstanding review requests by reviewer, computes how long each reviewer has
# been waiting, and builds a single Slack Block Kit message.
#
# If SLACK_WEBHOOK_URL is set, the message is posted to the channel.
# Otherwise the JSON payload is printed to stdout (useful for previewing in
# Slack's Block Kit Builder: https://app.slack.com/block-kit-builder).
#
# Requires: gh (authenticated, with read access to every repo in $REPOS),
# jq, and curl (only when posting). Block Kit construction lives in
# scripts/notify_pending_reviews_payload.jq.

set -euo pipefail

# --- Configuration -----------------------------------------------------------

# Repos to monitor (owner/name), one per line. Override via $REPOS.
DEFAULT_REPOS='
software-mansion/cairo-profiler
software-mansion/cairo-coverage
software-mansion/cairols
software-mansion/cairo-lint
software-mansion/vscode-cairo
software-mansion/cairo-language-common
software-mansion-labs/cairo-tm-grammar
software-mansion/scarb
software-mansion/scarbs.xyz
software-mansion/maat
software-mansion/cairo-oracle
software-mansion/scarb-nightlies
software-mansion-labs/scarb-eject
software-mansion/starkup
software-mansion/asdf-scarb
software-mansion/asdf-cairo-profiler
software-mansion/asdf-cairo-coverage
software-mansion/setup-scarb
software-mansion-labs/cairo-toolchain-xtasks
'

DEFAULT_TEAMS='
cairo-ls
scarb-maintainers
'

REPOS="${REPOS:-$DEFAULT_REPOS}"
# Comma-separated reviewer logins / team slugs to drop (e.g. catch-all teams
# that get auto-requested on every PR).
IGNORE_TEAMS="${IGNORE_TEAMS:-$DEFAULT_TEAMS}"
PR_FETCH_LIMIT="${PR_FETCH_LIMIT:-50}"
SLACK_WEBHOOK_URL="${SLACK_WEBHOOK_URL:-}"
GH_API_MAX_ATTEMPTS="${GH_API_MAX_ATTEMPTS:-3}"
GH_API_RETRY_DELAY_S="${GH_API_RETRY_DELAY_S:-2}"

# --- Dependency check --------------------------------------------------------

REQUIRED=(gh jq)
if [ -n "$SLACK_WEBHOOK_URL" ]; then
    REQUIRED+=(curl)
fi
for cmd in "${REQUIRED[@]}"; do
    if ! command -v "$cmd" &> /dev/null; then
        echo "❌ Error: $cmd not found. Please install it." >&2
        exit 1
    fi
done

gh_api_graphql_with_retry() {
    local repo="$1"
    local owner="${repo%/*}"
    local name="${repo#*/}"
    local attempt=1
    local delay="$GH_API_RETRY_DELAY_S"

    while [ "$attempt" -le "$GH_API_MAX_ATTEMPTS" ]; do
        if gh api graphql -f query="$QUERY" \
            -F owner="$owner" -F name="$name" -F prFetchLimit="$PR_FETCH_LIMIT" 2>/dev/null; then
            return 0
        fi

        if [ "$attempt" -eq "$GH_API_MAX_ATTEMPTS" ]; then
            break
        fi

        echo "⚠️  GitHub API query failed for $repo (attempt $attempt/$GH_API_MAX_ATTEMPTS); retrying in ${delay}s..." >&2
        sleep "$delay"
        attempt=$((attempt + 1))
        delay=$((delay * 2))
    done

    return 1
}

# --- Query -------------------------------------------------------------------

# Pull current pending reviewers + ReviewRequestedEvent timestamps via GraphQL.
# shellcheck disable=SC2016  # $owner/$name are GraphQL variables, not shell.
QUERY='
query($owner: String!, $name: String!, $prFetchLimit: Int!) {
  repository(owner: $owner, name: $name) {
    pullRequests(first: $prFetchLimit, states: OPEN, orderBy: {field: UPDATED_AT, direction: DESC}) {
      nodes {
        title
        url
        number
        isDraft
        additions
        deletions
        reviewRequests(first: 20) {
          nodes {
            requestedReviewer {
              __typename
              ... on User { login }
              ... on Team { slug }
            }
          }
        }
        timelineItems(itemTypes: [REVIEW_REQUESTED_EVENT], last: 100) {
          nodes {
            ... on ReviewRequestedEvent {
              createdAt
              requestedReviewer {
                __typename
                ... on User { login }
                ... on Team { slug }
              }
            }
          }
        }
      }
    }
  }
}
'

# Per-repo row transform: one object per (reviewer, PR) pair. Drafts are dropped.
ROW_FILTER='
  if .errors then error("GitHub API error for \($repo): \(.errors | tostring)") else . end
  | ($ignore_teams | split("[,[:space:]]+"; "") | map(select(length > 0))) as $ignore
  | [
      .data.repository.pullRequests.nodes[]
      | select(.isDraft | not)
      | . as $pr
      | ($pr.reviewRequests.nodes
          | map(.requestedReviewer | (.login // .slug))
          | map(select(. != null and (. as $r | $ignore | index($r) | not)))) as $pending
      | $pending[]
      | . as $reviewer
      | (
          $pr.timelineItems.nodes
          | map(select(.requestedReviewer != null
              and (.requestedReviewer.login // .requestedReviewer.slug) == $reviewer))
          | map(.createdAt | fromdateiso8601)
          | max
        ) as $ts
      | {
          repo: $repo,
          reviewer: $reviewer,
          title: $pr.title,
          url: $pr.url,
          number: $pr.number,
          waiting_s: (if $ts then ((now - $ts) | floor) else null end),
          additions: $pr.additions,
          deletions: $pr.deletions
        }
    ]
'

# --- Collect rows across all repos -------------------------------------------

echo "Fetching pending review requests..." >&2

ALL_ROWS='[]'
for repo in $REPOS; do
    [ -z "$repo" ] && continue
    if ! resp=$(gh_api_graphql_with_retry "$repo"); then
        echo "⚠️  Skipping $repo (query failed after retries — missing access or transient API failure?)" >&2
        continue
    fi
    rows=$(jq --arg repo "$repo" --arg ignore_teams "$IGNORE_TEAMS" \
        "$ROW_FILTER" <<<"$resp")
    ALL_ROWS=$(jq -n --argjson a "$ALL_ROWS" --argjson b "$rows" '$a + $b')
done

# Sort by reviewer asc, then waiting time desc (-1 guards against null).
ROWS_JSON=$(jq 'sort_by(.reviewer, -(.waiting_s // -1))' <<<"$ALL_ROWS")

# --- Build and deliver the Block Kit payload ---------------------------------

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PAYLOAD=$(jq -f "$SCRIPT_DIR/notify_pending_reviews_payload.jq" <<<"$ROWS_JSON")

if [ -n "$SLACK_WEBHOOK_URL" ]; then
    RESP=$(curl -sS --max-time 30 -w $'\n%{http_code}' -X POST \
        -H 'Content-Type: application/json' \
        --data "$PAYLOAD" "$SLACK_WEBHOOK_URL")
    HTTP_CODE=${RESP##*$'\n'}
    BODY=${RESP%$'\n'*}
    if [ "$HTTP_CODE" = "200" ]; then
        echo "✅ Sent to Slack (HTTP $HTTP_CODE)" >&2
    else
        echo "❌ Slack webhook failed (HTTP $HTTP_CODE): $BODY" >&2
        exit 1
    fi
else
    printf '%s\n' "$PAYLOAD"
    echo "✅ Block Kit payload built (set SLACK_WEBHOOK_URL to post to Slack)" >&2
fi
