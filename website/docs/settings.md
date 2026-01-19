# Settings

The Visual Studio Code extension provides multiple settings that let you customize various things related to your cairo code.

In order to access them, go to **Settings** -> **Extensions** -> **Cairo**.

## Test Runner

By default, it's set to `auto`, which means it will be automatically detected based on the `Scarb.toml`.
You can also select a `custom` test runner. In this case the **Run Test Command** field must be provided.

## Run Test Command

Custom command used to run tests. Also the `{{TEST_PATH}}` variable available.

Example: `snforge test {{TEST_PATH}} --exact`.

## Enable Linter

Enables [Cairo lint](https://docs.swmansion.com/cairo-lint/) diagnostics and code actions.

## Enable Proc Macros

Enables support for procedural macros. 

> [!WARNING]
> With Language Server version 2.15 and above, the proc macro cache will be enabled by default and the option to turn it off will no longer be available. 

## Scarb Path

Absolute path to the [Scarb](https://docs.swmansion.com/scarb/) binary.

**Default**: Scarb executable set in your `PATH` environment variable.

## Language Server Path

Absolute path to the Language Server binary. If specified, it will be used instead of the Scarb's language server.

**Default**: Provided by Scarb.

## Corelib Path

Absolute path to Cairo core library. It's necessary if the project doesn't use Scarb as a project manager.

**Default**: Provided by Scarb.

## Enable Proc Macro Cache (experimental)

Enables on disk cache for procedural macros.

> [!WARNING]
> With Language Server version 2.15 and above, this option has no effect, as the Proc Macro Cache is no longer experimental and is always enabled.

> [!WARNING]
> If you are using any Language Server version **below 2.15.0**, your cache will **NOT** be invalidated. If **ANY** of your procedural macro dependencies change - remove the `cairo-language-server/proc_macro.cache` file from your target directory.
