# Helix Setup for Leo Syntax Highlighting

This guide explains how to set up tree-sitter based syntax highlighting for Leo in Helix.

## Installation

### Step 1: Add Language Configuration

Add the following to your `~/.config/helix/languages.toml`:

```toml
[[language]]
name = "leo"
scope = "source.leo"
injection-regex = "leo"
file-types = ["leo"]
roots = ["program.json", "Leo.toml"]
comment-token = "//"
block-comment-tokens = { start = "/*", end = "*/" }
indent = { tab-width = 4, unit = "    " }
auto-format = false

[language.auto-pairs]
'(' = ')'
'{' = '}'
'[' = ']'
'"' = '"'

[[grammar]]
name = "leo"
source = { git = "https://github.com/ProvableLabs/leo", subpath = "tree-sitter-leo", rev = "master" }
```

### Step 2: Build the Grammar

```bash
hx --grammar fetch
hx --grammar build
```

### Step 3: Copy Query Files

```bash
mkdir -p ~/.config/helix/runtime/queries/leo
cp /path/to/leo/tree-sitter-leo/queries/*.scm ~/.config/helix/runtime/queries/leo/
```

Or create symlinks:

```bash
mkdir -p ~/.config/helix/runtime/queries
ln -s /path/to/leo/tree-sitter-leo/queries ~/.config/helix/runtime/queries/leo
```

## Verify Installation

1. Open a `.leo` file with Helix
2. Press `:` and type `tree-sitter-highlight-name` to see highlighting info
3. Use `:tree-sitter-subtree` to inspect the syntax tree

## Customizing Colors

Helix uses themes to define colors. The Leo queries use standard capture names that should work with any Helix theme.

Key captures used:
- `@keyword` - Language keywords
- `@type` - Type names  
- `@type.builtin` - Built-in types (u32, bool, etc.)
- `@function` - Function definitions
- `@function.call` - Function calls
- `@variable` - Variables
- `@constant` - Constants
- `@string` - String literals
- `@number` - Numeric literals
- `@comment` - Comments
- `@operator` - Operators
- `@punctuation.bracket` - Brackets
- `@punctuation.delimiter` - Delimiters

## Example Theme Additions

If you want custom colors for Leo, add to your theme:

```toml
# In ~/.config/helix/themes/my-theme.toml

"type.builtin" = { fg = "cyan", modifiers = ["bold"] }
"variable.builtin" = { fg = "magenta" }
"namespace" = { fg = "yellow" }
"string.special" = { fg = "green", modifiers = ["italic"] }
```

## Troubleshooting

### Grammar not building

Make sure you have the required build tools:
- A C compiler (gcc or clang)
- Node.js (for tree-sitter CLI)

### Highlights not showing

Check that queries are in the right location:

```bash
ls ~/.config/helix/runtime/queries/leo/
# Should show: highlights.scm, locals.scm, tags.scm
```

### Wrong file type detection

Ensure the `file-types` in `languages.toml` includes `"leo"`.
