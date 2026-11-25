# CairoLS Architecture

## Bird's eye view

```mermaid
flowchart LR
    editor(["Editor<br/>(e.g. VS Code extension)"])
    subgraph LS ["CairoLS"]
        direction TB
        server["server<br/>(event loop, scheduling, routing)"]
        ide["ide<br/>(LSP features)"]
        lang["lang<br/>(analysis database, diagnostics, proc macros)"]
        project["project<br/>(workspace model)"]
        server --> ide --> lang
        server --> project --> lang
    end
    compiler[["cairo-lang-* crates<br/>(Cairo compiler)"]]
    lint[["cairo-lint"]]
    scarb(["scarb metadata"])
    pms(["scarb proc-macro-server"])

    editor <-- "LSP over stdio" --> server
    lang --> compiler
    lang --> lint
    project -- "spawns" --> scarb
    lang <-- "JSON-RPC over stdio" --> pms
```

CairoLS is a [Language Server Protocol](https://microsoft.github.io/language-server-protocol/) implementation for Cairo.
It is shipped with [Scarb] and started by editors as `scarb cairo-language-server` or `cairo-language-server`.

CairoLS does not have its own compiler front-end.
It runs the Cairo compiler crates (`cairo-lang-*`) on a [Salsa](https://github.com/salsa-rs/salsa) database.
On top of that it adds:

- a project model built from `Scarb.toml` (via `scarb metadata`) or `cairo_project.toml`,
- procedural macro support through [Scarb]'s proc-macro-server,
- LSP features (completion, hover, navigation, ...) implemented as queries on that database.

Arrows in the diagram show the main direction of use, not strict layering.
The server runtime (event loop, task scheduling, routing) is adapted from [ruff_server](https://github.com/astral-sh/ruff/tree/main/crates/ruff_server).

## Code map

| Path                | Responsibility                                                                                                 |
|---------------------|----------------------------------------------------------------------------------------------------------------|
| `src/lib.rs`        | Entry point (`start()`), `Backend`, the main event loop.                                                       |
| `src/state.rs`      | `State` (owned by the main thread), `StateSnapshot` (read-only, used by background tasks), `MetaState`.        |
| `src/server/`       | LSP connection, scheduler and thread pools, request/notification routing and handlers.                         |
| `src/lsp/`          | Server capabilities, CairoLS-specific LSP extensions (`ext.rs`).                                               |
| `src/config.rs`     | Client configuration (`workspace/configuration`), can change at runtime.                                       |
| `src/env_config.rs` | Configuration through environment variables (logging, profiling, tuning).                                      |
| `src/project/`      | Discovering and loading workspaces, applying crates and their configs to the database.                         |
| `src/toolchain/`    | Locating and running [Scarb].                                                                                  |
| `src/lang/`         | `AnalysisDatabase`, symbol resolution, find usages, diagnostics, proc macros, LSP ↔ Cairo conversions.         |
| `src/ide/`          | One module per LSP feature: completion, hover, navigation, code actions, code lens, formatting, inlay hints... |
| `tests/e2e/`        | End-to-end test suite with a mock language client.                                                             |
| `tests/scarb_*`     | Example procedural macros (v1 and v2 API) used by tests.                                                       |
| `xtask/`            | Maintenance scripts (`cargo xtask upgrade`, `cargo xtask sync-version`).                                       |
| `website/`          | User documentation site.                                                                                       |

## Analysis database

```mermaid
flowchart TB
    db[("AnalysisDatabase")]
    subgraph ls ["CairoLS groups (src/lang)"]
        lssyntax["LsSyntaxGroup"]
        lssemantic["LsSemanticGroup"]
        lsproto["LsProtoGroup"]
        procmacro["ProcMacroGroup"]
    end
    common["CommonGroup<br/>(cairo-language-common)"]
    lint["LinterGroup<br/>(cairo-lint)"]
    subgraph compiler ["Cairo compiler (cairo-lang-*)"]
        files["FilesGroup"] --> parser["ParserGroup"] --> defs["DefsGroup"] --> semantic["SemanticGroup"] --> lowering["LoweringGroup"]
    end
    db --> ls & common & lint
    ls & common & lint --> compiler
```

`AnalysisDatabase` (`src/lang/db/`) is a single Salsa database that holds only storage.
All functionality comes from extension traits ("groups") implemented for any Salsa database.
Arrows mean "builds on".
`CommonGroup` is shared with cairo-lint.
The LS-specific groups are:

- `LsSyntaxGroup`: finding syntax nodes at a position,
- `LsSemanticGroup`: mapping syntax nodes to semantic items,
- `LsProtoGroup`: mapping between LSP URLs and compiler files (including virtual ones),
- `ProcMacroGroup`: proc-macro server state and cached expansions.

New database functionality should be added as such a group, not as fields on `AnalysisDatabase`.

### Compiler plugins

Each [Scarb] crate gets the same compiler plugins, in the same order, as [Scarb] would use.
`cairo_project.toml` projects and detached files use a default plugin set.
Proc-macro plugins are added when the proc-macro server reports the available macros, and removed when it restarts.

## `lang` and `ide`

- `lang` is the semantic toolkit built on the database.
  The most important parts:
  - `lsp`: conversions between LSP and compiler types (positions, spans, locations),
  - `defs`: resolving the symbol at a position (`SymbolSearch` → `SymbolDef`),
  - `usages`: finding references of a `SymbolDef`,
  - smaller helpers used by completion and code actions.
- `ide` has one module per LSP feature.
  A feature is mostly a function of request params and `&AnalysisDatabase` that returns the LSP response.
  Features are mostly stateless; the few that keep state use a controller owned by `State`.

A typical request, for example goto definition:

1. Convert the LSP URL and position to a compiler file and offset (`LsProtoGroup`, `lang::lsp`).
2. Find the syntax node at that offset (`LsSyntaxGroup`).
3. Map it to its *resultants*: the matching nodes in macro-generated code (`CommonGroup`).
4. Resolve the symbol (`lang::defs`), which uses compiler semantic queries.
5. Map the result back to user code and convert it to an LSP location.

## Runtime

### Threads and the event loop

```mermaid
flowchart LR
    client(["LSP client"])
    evloop["Main thread<br/>event loop<br/>(owns &mut State)"]
    pools["Thread pools<br/>(requests on StateSnapshot)"]
    controllers["Controller threads<br/>(project, diagnostics, proc macros, ...)"]

    client -- "messages" --> evloop
    evloop -- "tasks" --> pools
    pools -- "responses" --> client
    evloop <-- "channels" --> controllers
```

The main thread runs a single event loop which waits on several channels:
LSP messages, project updates, proc-macro responses, analysis status, retries and a few others.
Project updates are handled first, because they change the state the most.

Requests run on thread pools.
Controller threads load projects, compute diagnostics and poll for proc-macro responses.
Threads communicate mostly through channels.
Apart from that, they share database clones (see [Cancellation](#cancellation)) and a little state behind a mutex (`MetaState`).

### Tasks

Every incoming message becomes a `Task` (see `src/server/schedule/`):

| Task kind    | Runs on        | Access              | Used for                                                    |
|--------------|----------------|---------------------|-------------------------------------------------------------|
| `SyncMut`    | main thread    | `&mut State`        | Notifications (`didOpen`, `didChange`, ...), state changes. |
| `Sync`       | main thread    | `&State`            | Small read-only actions.                                    |
| `Background` | worker pool    | `StateSnapshot`     | Almost all LSP requests (hover, completion, ...).           |
| `Fmt`        | fmt pool       | `StateSnapshot`     | Formatting, so it is never blocked by other requests.       |

Request and notification handlers are registered in `src/server/routing/`.

After every `SyncMut` task that changed the database or config, the scheduler runs hooks which:

1. may swap the database (see [Memory management](#memory-management)),
2. refresh diagnostics,
3. mark that analysis is in progress.

Tasks do not need to schedule these themselves.

### Cancellation

```mermaid
sequenceDiagram
    participant C as Client
    participant M as Main thread
    participant W as Worker
    C->>M: textDocument/completion
    M->>W: Background task with StateSnapshot (db clone)
    C->>M: textDocument/didChange
    M->>M: SyncMut: write to db
    Note over W: Salsa cancels the task
    alt RETRY = true (e.g. completion)
        W->>M: retry (handler with the original id and params)
        M->>W: Background task with a fresh StateSnapshot
        W->>C: completion response
    else RETRY = false
        W->>C: ServerCancelled error
    end
```

CairoLS cancels from the inside.
Any write to the database makes Salsa cancel queries running on all snapshots.
The write waits until all other database clones are dropped.
Handlers catch the cancellation:
requests marked `RETRY = true` (e.g. completion, semantic tokens) are retried by the server on a fresh snapshot,
so the client does not have to send them again.
Others answer with `ServerCancelled`.
`$/cancelRequest` from the client is ignored.

### File changes

Document notifications run as `SyncMut` tasks:

- `didOpen` / `didChange` set the editor content as a file override in the database (full sync).
- `didOpen` also asks the project controller to load the project for the file.
- `didSave` / `didClose` remove the override, so the database reads the file from disk again.
- Salsa does not track reads from disk.
  `didSave` and watched `.cairo` file changes do a synthetic write (`cancel_all`) to invalidate them.
- Watched file changes to `Scarb.toml` / `cairo_project.toml` reload the config and the project model and restart the proc-macro server.

## Project model

```mermaid
flowchart TB
    open["didOpen / manifest change"] --> ctrl["project controller thread"]
    ctrl --> discover{"Find manifest"}
    discover -- "cairo_project.toml" --> cpt["cairo project"]
    discover -- "Scarb.toml" --> sm["scarb metadata"] --> scarb["Scarb workspace"]
    discover -- "none" --> none["detached file"]
    cpt & scarb & none -- "ProjectUpdate" --> apply["main thread applies update"]
    apply --> db[("AnalysisDatabase<br/>crates, configs, plugins")]
```

`src/project/` finds the project of every opened file and loads its crates into the database.
Loading runs on a separate thread because `scarb metadata` is slow.
The result is applied to the database on the main thread.

- The controller looks in parent directories for `cairo_project.toml` and `Scarb.toml`.
  If both exist, `cairo_project.toml` wins.
- Workspaces that are already loaded and files from the [Scarb] cache are skipped.
- `ProjectModel` keeps all loaded workspaces, because one LS session can have many of them.
  `ConfigsRegistry` keeps per-package settings, such as `[tool.cairo-lint]`.
- An unmanaged corelib is set up if no `core` crate is loaded:
  for detached files, `cairo_project.toml` projects, or when `scarb metadata` fails.
- [Scarb] manifest errors are published as diagnostics on `Scarb.toml`.

## Diagnostics

```mermaid
flowchart LR
    main[("AnalysisDatabase<br/>(main thread)")] -- "mutation hook<br/>copies the inputs" --> copy[("diagnostics database<br/>(fresh copy)")]
    copy --> ctrl["diagnostics<br/>controller"]
    ctrl -- "batches of files" --> workers["worker pool"]
    workers -- "publishDiagnostics<br/>(changed files only)" --> client(["Client"])
    workers <--> pd[("ProjectDiagnostics")]
```

Diagnostics are computed for the whole project, not only for open files (`src/lang/diagnostics/`):

1. After each mutation, the main thread copies the inputs of its database into a fresh one
   (see [Memory management](#memory-management)) and hands the copy to the diagnostics controller.
   A newer refresh cancels the one that is running.
2. Open files are checked first, then all other files of the project.
   Files are split into batches processed by a small worker pool.
3. Each file collects parser, semantic and lowering diagnostics.
   If the linter is enabled, cairo-lint diagnostics are added too.
   Dependencies from the [Scarb] cache only report errors.
4. `ProjectDiagnostics` keeps the last published diagnostics per file, so only changes are sent.

When the whole analysis is finished, the main loop saves the proc-macro expansion cache
(see [Procedural macros](#procedural-macros)) and asks the client to refresh code lenses and semantic tokens.

## Procedural macros

```mermaid
sequenceDiagram
    participant Q as Salsa query<br/>(compiler plugin)
    participant DB as ProcMacroGroup
    participant PMS as scarb proc-macro-server
    participant M as Main thread
    Q->>DB: expand macro
    DB->>PMS: send request (not cached yet)
    DB-->>Q: placeholder (code unchanged)
    PMS-->>M: responses (polled in batches)
    M->>DB: store expansions (db mutation)
    Note over Q,M: diagnostics refresh, queries re-run
    Q->>DB: expand macro
    DB-->>Q: cached expansion
```

`src/lang/proc_macros/` integrates procedural macros:

- `ProcMacroClientController` starts `scarb proc-macro-server` and restarts it after failures (rate-limited).
- The server status and all expansion results are a Salsa input (`ProcMacroGroup`).
- Compiler plugins never wait for the server.
  On a cache miss they send a request and return a placeholder.
- The main thread periodically collects responses and writes them into the database in one go.
  This is a mutation, so analysis runs again.
- Expansions are also cached on disk in the `target` directory, so a new LS session starts faster.

## Memory management

Salsa never frees memoized results, so memory grows during long sessions.
`AnalysisDatabaseSwapper` (`src/lang/db/swapper.rs`) replaces the whole database with a fresh one:
after a number of mutations, or after a period of inactivity.

`src/lang/db/migration.rs` builds the fresh database.
It copies only the inputs: plugins, proc-macro state, file overrides and crate configuration.
The diagnostics controller uses the same migration for each refresh.

## Testing

- E2E tests live in `tests/e2e/`, one directory per feature, all compiled into one test binary.
- `sandbox!` creates a temporary project and starts the server with an in-memory connection and a `MockClient`.
  It disables proc macros and the linter unless the test's workspace configuration enables them.
- Most tests use inline [insta](https://insta.rs/) snapshots.
  A feature implements the `Transformer` trait, then tests use `test_transform_plain!` or `test_transform_with_macros!`.
- `test_transform_with_macros!` runs against the example macros in `tests/scarb_procedural_macros_v2`.
- The `testing` cargo feature exposes test-only parts of the server (e.g. `BackendForTesting`).

## Dependencies and versioning

- CairoLS is versioned together with the Cairo compiler.
- `cairo-lang-*`, `cairo-lint` and related crates are always declared as crates.io dependencies.
  During development they can be pinned to git revisions in `[patch.crates-io]`.
  [Scarb], which ships CairoLS, sets the final revisions for the whole toolchain.
- Use `cargo xtask upgrade` to change these dependencies.
  See [MAINTAINING.md](./MAINTAINING.md) for the release process.

## Rules of thumb

- Only `SyncMut` tasks on the main thread mutate `State` or the database.
  They block the event loop, so keep them short.
  Slow work goes to a controller thread and returns to the main loop through a channel.
- A database write waits until all clones are dropped.
  Do not keep database clones longer than needed.
- Cancellation is not an error.
  If you catch panics, detect Salsa cancellation with `is_cancelled` and re-raise it.
- IDs like `FileId` or `CrateId` belong to one database instance.
  They are invalid after a swap and in the diagnostics database.
  State kept outside the database uses stable keys, such as `Url` or `CrateInput`.
- Database inputs set at runtime must be copied in `migration.rs` ([why](#memory-management)).
- Code may live in macro-generated virtual files.
  Map nodes and locations between user code and generated code (e.g. `get_node_resultants`, `get_originating_location`).
- `Config` can change at runtime.
  Read it from `State` or `StateSnapshot` instead of keeping your own copy.

[Scarb]: https://github.com/software-mansion/scarb
