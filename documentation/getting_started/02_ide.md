---
id: ide
title: Your Development Environment
sidebar_label: Dev Env
---

[general tags]: # "ide, plugin"

Developers can choose from a wide variety of development environments.

## Language Server

Leo ships a Language Server Protocol (LSP) implementation (`leo-lsp`) that powers the editor plugins listed below. The server exposes the following capabilities:

- **Semantic highlighting** — token classification driven by the compiler's own analysis, so keywords, types, and identifiers are highlighted consistently with how Leo actually parses your code.
- **Push diagnostics** — compile errors and warnings (including the CEI analysis warnings described in the [Finalization Model guide](../guides/01_finalization.md#checks-effects-interactions-cei)) are surfaced inline as you edit, using the same ariadne-rendered messages you see on the command line.
- **Go to definition** — jump from any identifier to where it is defined, including across module and library boundaries.
- **Find all references** — list every use of a symbol across the package.
- **Rename** — rename a symbol everywhere it is used. The server uses `prepare-rename` to validate the target before applying the edit.

The server does not currently provide hover, completion, or code actions. These are tracked for future releases.

## Plugins

<!--TODO: Condense this.--->

The Leo team maintains editor clients under [`ProvableHQ/leo-lsp-clients`](https://github.com/ProvableHQ/leo-lsp-clients) — every client launches the same `leo-lsp` server, so the capabilities above apply uniformly. If you do not see your favorite editor on this list, please reach out on [GitHub](https://github.com/ProvableHQ/leo/issues/new).

### VS Code

[//]: # "![](./images/vscode.png)"

Download the editor here: <https://code.visualstudio.com/download>.

#### Install

1. Install [Leo for VSCode](https://marketplace.visualstudio.com/items?itemName=aleohq.leo-extension) from the VSCode marketplace.
2. The correct extension ID is `aleohq.leo-extension`, and the description should state "the official VSCode extension for Leo".

#### Usage

1. Open `VSCode`.
2. Go to Settings > Extensions or use the left side panel Extensions button to enable the Leo plugin.

### Cursor

[Cursor](https://www.cursor.com/) reuses the VSCode extension surface. The Leo client is implemented and pending publication to [Open VSX](https://open-vsx.org/); once published, install it from Cursor's Extensions panel by searching for "Leo". Progress is tracked in [leo-lsp-clients#10](https://github.com/ProvableHQ/leo-lsp-clients/issues/10).

### Google Antigravity

[Antigravity](https://antigravity.google) is also a VSCode-compatible host and is supported by the same client package. Like Cursor, the client is implemented and pending marketplace publication — track availability in [leo-lsp-clients#10](https://github.com/ProvableHQ/leo-lsp-clients/issues/10).

### Sublime Text

[//]: # "![](./images/sublime.png)  "

Download the editor here: <https://www.sublimetext.com/download>.
Aleo instruction support for Sublime's LSP plugin is provided through a language-server.

#### Install

1. Install [LSP](https://packagecontrol.io/packages/LSP) and [LSP-leo](https://packagecontrol.io/packages/LSP-leo) from Package Control.
2. Restart Sublime.

#### Usage

Follow these steps to toggle the `Leo` syntax highlighting, hover, and tokens.

1. Open `Sublime Text`.
2. From Settings > Select Color Scheme... > LSP-leo

### Intellij

[//]: # "![](./images/intellij.png)"

Download the editor here: <https://www.jetbrains.com/idea/download/>.

#### Install

1. Install and enable the Leo [plugin](https://plugins.jetbrains.com/plugin/19979-leo) in your IDE.
