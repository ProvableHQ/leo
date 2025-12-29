-- Neovim configuration for Leo language support
-- Add this to your init.lua or a separate plugin file

-- Register Leo filetype
vim.filetype.add({
  extension = {
    leo = "leo",
  },
  pattern = {
    [".*%.leo"] = "leo",
  },
})

-- Configure tree-sitter parser
local ok, parser_config = pcall(require, "nvim-treesitter.parsers")
if ok then
  parser_config = parser_config.get_parser_configs()
  parser_config.leo = {
    install_info = {
      url = "https://github.com/ProvableLabs/leo",
      files = { "tree-sitter-leo/src/parser.c" },
      location = "tree-sitter-leo",
      branch = "master",
      generate_requires_npm = false,
      requires_generate_from_grammar = false,
    },
    filetype = "leo",
    maintainers = { "@ProvableLabs" },
  }
end

-- Set up comment string for Leo
vim.api.nvim_create_autocmd("FileType", {
  pattern = "leo",
  callback = function()
    vim.bo.commentstring = "// %s"
    vim.bo.comments = "s1:/*,mb:*,ex:*/,:///,://"
  end,
})

-- Optional: Set up LSP configuration (if Leo LSP exists)
-- vim.api.nvim_create_autocmd("FileType", {
--   pattern = "leo",
--   callback = function()
--     vim.lsp.start({
--       name = "leo-lsp",
--       cmd = { "leo", "lsp" },
--       root_dir = vim.fs.dirname(vim.fs.find({ "program.json", "Leo.toml" }, { upward = true })[1]),
--     })
--   end,
-- })

-- Custom highlight links for Leo
local function setup_highlights()
  local links = {
    ["@keyword.coroutine.leo"] = "Keyword",
    ["@variable.builtin.leo"] = "Special",
    ["@string.special.leo"] = "String",
    ["@namespace.leo"] = "Include",
    ["@type.definition.leo"] = "Type",
    ["@property.definition.leo"] = "Identifier",
    ["@field.leo"] = "Identifier",
    ["@conditional.ternary.leo"] = "Conditional",
  }

  for from, to in pairs(links) do
    vim.api.nvim_set_hl(0, from, { link = to, default = true })
  end
end

-- Apply highlights on colorscheme change
vim.api.nvim_create_autocmd("ColorScheme", {
  callback = setup_highlights,
})

-- Apply highlights immediately
setup_highlights()

return {
  -- Export for use as a module
  setup = function(opts)
    opts = opts or {}
    if opts.highlights ~= false then
      setup_highlights()
    end
  end,
}
