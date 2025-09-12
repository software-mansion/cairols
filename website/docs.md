# Quick start

Cairo Language Server is distributed along with the [Scarb](https://docs.swmansion.com/scarb) package manager. 

## Installation

To make the Language Server work, you have to install:
1. [Scarb](https://docs.swmansion.com/scarb/download.html) package manager, which includes the Cairo Language Sever. Make sure you verify the installation by running:
```shell
scarb --version
```
2. For Visual Studio Code, install
the Visual Studio Code [Cairo 1.0](https://marketplace.visualstudio.com/items?itemName=starkware.cairo1) extension
from the marketplace.
3. For other editors that support [Language Server Protocol](https://microsoft.github.io/language-server-protocol/),
you can run and point the editor to this command to start the server.
```shell
scarb cairo-language-server
``` 




