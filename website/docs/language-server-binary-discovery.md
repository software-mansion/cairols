# Language Server Binary Discovery

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

> [!WARNING]
> You should **not** rely on this behavior; use configuration or the `PATH` environment variable
> instead!

Should the search in `PATH` fail, the extension employs a more involved logic to locate the `asdf`
version manager tool on your machine. This step accounts for instances where
[Scarb](https://docs.swmansion.com/scarb/) is installed via `asdf` but `asdf` is not available in
the system `PATH`.
The extension attempts to execute `asdf` directly to determine the installed path of
[Scarb](https://docs.swmansion.com/scarb/) and to use the associated Language Server binary.
