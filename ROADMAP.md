# Cairo Language Server Roadmap

This document aims to serve as a place where high level information about upcoming, planned work for CairoLS lives.
It is basically a list of main features and tasks that are planned to be added in the next ~6 months. 

The list is sorted from top to bottom according to the priority the given task has (first thing having the top priority,
ie being more likely to be done first). Size of the task is ranked `1-5`, where `1` is smallest and `5` biggest.

A more detailed breakdown can be found on the [Kanban board](https://github.com/orgs/software-mansion/projects/33/views/6),
typically in the `Todo` and `Backlog LS` columns.

## Q4 2025 + Q1 2026

### 1. Fixing bugs

We have a few smaller bugs that need to be addressed - ranging from completions applying wrong edits (full paths instead of part of it),
to lack of support for finding references for struct fields, when invoking on the struct itself.

Size: 5 overall; 1-2 (per task)

### 2. Macros improvements

The aim is, on one hand, to refactor and upstream parts of the code borrowed from scarb, which would make maintenance a lot easier.
On the other hand, this also includes things like proper invalidation for proc macro cache, support code lenses in declarative macros
and support for expanding the declarative macros.

Size: 4

### 3. Improvements to comment/comment blocks and documentation

We should support specialized completions for links in doc comments and cross-item linking in the hover docs.

Size: 2

### 4. Features around `Scarb.toml` files

We want to start showing hovers with documentation for `Scarb.toml` fields as well as add completions to its syntax.

Size: 2

### 5. Hovers

We could add more hovers, that would serve specific needs - e.g. identify the expression type on hover, or informing about the package from which the derive macro comes.

Size: 3

### 6. Improvements to completions

Completions could be smarter (ie could be showing in function body, when nothing is typed; shouldn't propose items that are already in scope etc.).
Apart from that, we should also support path completions for trait/impl items, and only suggest types that make sense for type annotations.

Size: 3

### 7. Additional commands

We could support running `cairo-profiler` and `cairo-coverage` through code lenses.

Size: 4

### 8. Extract/inline refactoring

We could suggest extract/introduce/inline refactoring of variables and functions/methods to users, where parts of the repeated code
can be grouped and deduplicated by placing them into separate functions/methods or just extracted to a variable (and the other way around, in case of inline).

Size: 4
