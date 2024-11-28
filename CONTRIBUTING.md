# Contributing to CairoLS

CairoLS is actively developed and open for contributions!

*Want to get started?*
Grab any unassigned issue labelled with [
`help wanted`](https://github.com/software-mansion/cairols/labels/help%20wanted)!

*Looking for some easy warmup tasks?*
Check out issues labelled with [
`good first issue`](https://github.com/software-mansion/cairols/labels/good%20first%20issue)!

When contributing to this repository, please first discuss the change you wish to make via issue,
email, or any other method with the owners of this repository before making a change.

## Environment setup

The latest stable Rust is the only thing you really need.
It is recommended to use [rustup](https://rustup.rs/) for getting it.

This document assumes that you are using Visual Studio Code with
the [Cairo extension](https://marketplace.visualstudio.com/items?itemName=starkware.cairo1) as the
editor of choice for running CairoLS against.
You might also want to check out
the [Cairo extension's contributing guidelines](https://github.com/software-mansion/vscode-cairo/blob/main/CONTRIBUTING.md).

## Testing

We are building an extensive end-to-end tests suite for
CairoLS [here](crates/cairo-lang-language-server/tests/e2e).
These tests implement a simple language client that you can control (like put a cursor at certain
position, send a request to the server, etc.).
Check out existing tests for examples of what you can do.
If you need, don’t hesitate to extend the language client with new capabilities!
Its source code is located [here](crates/cairo-lang-language-server/tests/e2e/support/mod.rs).

> [!IMPORTANT]
> The test suite is not complete, but we **require** adding tests for any new developments.

Mind that these tests tend to be slow, so try to stuff as much as possible into a single test,
to reduce the overhead of constant sections, like booting LS or analysing the `core` crate.

## Debugging

This section shows some more advanced tricks that you can employ to debug the CairoLS.

### Advanced logging

You can enable more [granular][env-filter-directives] logging by configuring environment variables
for the language server.
To do so, paste the following into your `.vscode/settings.json`:

```json
{
    "cairo1.languageServerExtraEnv": {
        "CAIRO_LS_LOG": "cairo_lang_language_server=debug",
        "RUST_BACKTRACE": "1"
    }
}
```

### Profiling

CairoLS has built-in support for generating profile files based on its tracing/logging system.
This mechanism allows investigating various cases of slow query execution, deadlocks, and other
performance issues.

To generate a profile file, paste the following into your `.vscode/settings.json`:

```json
{
    "cairo1.languageServerExtraEnv": {
        "CAIRO_LS_PROFILE": "1"
    }
}
```

This will generate a trace file that you'll be able to further analyse.
CairoLS will print the path to this trace file and instructions on how to analyse it on its standard
error.
In Visual Studio Code you will find this output in the `Output` → `Cairo Language Server` panel.
We're not copying these here because nobody will bother keeping this document in sync.

### Use tests

If you find a short reproduction of your problem, we strongly suggest writing an E2E test and
including it in your PR.
Not only will this make your development cycle faster (because checking your changes will be now
automated),
but you will also enable future developers not to fall into the pitfall that caused the bug you
found and debugged 🤓.

## Git

Try to make small PRs that could be squashed into a single commit.
For larger work, try to make your commits small, self-contained, and well-described.
Each commit should pass lints and tests.
Then, set up a stack of pull requests, separate PR for each commit, and pointing to the previous
one.

While your PR is being reviewed on, you can push merge commits and use [
`git commit --fixup`](https://git-scm.com/docs/git-commit/2.32.0#Documentation/git-commit.txt---fixupamendrewordltcommitgt)
to push further changes to your commits.

## Typos

Our policy is to not accept PRs that only fix typos in the documentation and code.
We appreciate your effort, but we encourage you to focus on bugs and features instead.

---

Thanks! ❤️ ❤️ ❤️

CairoLS Team

[env-filter-directives]: https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
