# Neovim Setup for Leo Syntax Highlighting

This guide explains how to set up tree-sitter based syntax highlighting for Leo in Neovim.

## Prerequisites

- Neovim 0.9+ (with tree-sitter support)
- [nvim-treesitter](https://github.com/nvim-treesitter/nvim-treesitter) plugin

## Installation

### Option 1: Using nvim-treesitter (Recommended)

Add the following to your Neovim configuration (e.g., `init.lua`):

```lua
local parser_config = require("nvim-treesitter.parsers").get_parser_configs()

parser_config.leo = {
  install_info = {
    url = "https://github.com/ProvableLabs/leo",
    files = { "tree-sitter-leo/src/parser.c" },
    location = "tree-sitter-leo",
    branch = "master",
  },
  filetype = "leo",
}

-- Register the filetype
vim.filetype.add({
  extension = {
    leo = "leo",
  },
})
```

Then install the parser:

```vim
:TSInstall leo
```

### Option 2: Manual Installation

1. Clone and build the parser:

```bash
git clone https://github.com/ProvableLabs/leo
cd leo/tree-sitter-leo
npm install
npm run generate
```

2. Copy the queries to your Neovim runtime:

```bash
mkdir -p ~/.config/nvim/queries/leo
cp queries/*.scm ~/.config/nvim/queries/leo/
```

3. Add the parser to tree-sitter:

```lua
vim.treesitter.language.register("leo", "leo")

-- Point to the compiled parser
local parser_path = vim.fn.expand("~/path/to/leo/tree-sitter-leo")
vim.opt.runtimepath:append(parser_path)
```

## Configuration

### Enable Highlighting

```lua
require("nvim-treesitter.configs").setup({
  ensure_installed = { "leo" },
  highlight = {
    enable = true,
  },
})
```

### Recommended Color Scheme Mappings

If your color scheme doesn't have specific Leo highlights, you can add fallbacks:

```lua
-- Link Leo-specific highlights to standard ones
vim.api.nvim_set_hl(0, "@type.builtin.leo", { link = "Type" })
vim.api.nvim_set_hl(0, "@keyword.coroutine.leo", { link = "Keyword" })
vim.api.nvim_set_hl(0, "@variable.builtin.leo", { link = "Special" })
vim.api.nvim_set_hl(0, "@string.special.leo", { link = "String" })
vim.api.nvim_set_hl(0, "@namespace.leo", { link = "Include" })
```

## Verify Installation

1. Open a `.leo` file
2. Run `:InspectTree` to see the syntax tree
3. Run `:Inspect` on any token to see its highlight groups

## Troubleshooting

### Parser not found

Make sure the parser is compiled and accessible:

```vim
:echo nvim_get_runtime_file("parser/leo.so", v:false)
```

### Highlights not working

Check if highlights are loaded:

```vim
:echo nvim_get_runtime_file("queries/leo/highlights.scm", v:false)
```
