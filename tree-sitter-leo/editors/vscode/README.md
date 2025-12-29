# VS Code Setup for Leo Syntax Highlighting

VS Code doesn't natively support tree-sitter grammars, but there are several options for Leo syntax highlighting.

## Option 1: TextMate Grammar (Recommended for VS Code)

VS Code uses TextMate grammars for syntax highlighting. A TextMate grammar file is provided in this directory.

### Installation

1. Create or update the Leo extension directory:

```bash
mkdir -p ~/.vscode/extensions/leo-language
```

2. Copy the grammar files:

```bash
cp syntaxes/leo.tmLanguage.json ~/.vscode/extensions/leo-language/syntaxes/
cp package.json ~/.vscode/extensions/leo-language/
```

3. Restart VS Code

### Or install from the marketplace

If a Leo extension is available on the VS Code marketplace, install it directly:

```
ext install provable.leo-language
```

## Option 2: Using vscode-anycode

The [vscode-anycode](https://marketplace.visualstudio.com/items?itemName=AnyCode.anycode) extension provides tree-sitter based highlighting.

1. Install the anycode extension
2. Add tree-sitter-leo as a custom grammar (see anycode documentation)

## Option 3: Native Tree-sitter (Experimental)

VS Code is working on native tree-sitter support. When available, the tree-sitter grammar can be used directly.

## Files Provided

- `package.json` - VS Code extension manifest
- `syntaxes/leo.tmLanguage.json` - TextMate grammar for Leo
- `language-configuration.json` - Language configuration (brackets, comments, etc.)

## Building the Extension

To package as a VSIX:

```bash
cd editors/vscode
npm install -g vsce
vsce package
```

Then install the resulting `.vsix` file in VS Code.
