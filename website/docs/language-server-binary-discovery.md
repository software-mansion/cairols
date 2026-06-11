# Language Server Binary Discovery

> [!NOTE]
> This page describes how the [Visual Studio Code Cairo extension](https://marketplace.visualstudio.com/items?itemName=starkware.cairo1)
> discovers the Language Server binary. Other editors may use different mechanisms.

Commonly, the Cairo Language Server is distributed with [Scarb](https://docs.swmansion.com/scarb/)
and can be accessed using the `scarb cairo-language-server` command. However, settings are
available to use a standalone binary.

## Configuration-specified Path

First, the extension checks if a path to the Language Server binary has been explicitly specified
in the configuration settings under `cairo1.languageServerPath`.
If a path is specified, the extension attempts to use the binary at this location, unless the
`cairo1.preferScarbLanguageServer` setting is set to `true`.
It will _not_ fall back to other methods if this path is incorrect!

## Scarb Binary Discovery

If `cairo1.languageServerPath` is unset (default), the extension then searches for the
[Scarb](https://docs.swmansion.com/scarb/) binary in the `cairo1.scarbPath` setting.
It will _not_ fall back to other methods if this path is incorrect!
If `cairo1.scarbPath` is unset, then it searches in your system's `PATH` environment variable.

## Scarb Discovery via ASDF

> [!NOTE]
> There is no `asdf` for Windows, so this step will not occur there.

If [Scarb](https://docs.swmansion.com/scarb/) is not found in `PATH`, the extension falls back to
looking for the Scarb shim directly in the `asdf` shims directory. This is
the default behavior when Scarb is installed via `asdf` (e.g. using
[starkup](https://github.com/software-mansion/starkup)) but the shims directory is not on the
shell `PATH` — for example, when the editor's environment does not source the `asdf` init script.
The extension checks the path given by the `ASDF_DATA_DIR` environment variable, or
`~/.asdf/shims/scarb` by default.
