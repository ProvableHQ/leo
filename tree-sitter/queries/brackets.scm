; Bracket pairs for Leo.
;
; Consumed by editors that match/auto-surround delimiters from tree-sitter
; (e.g. Zed's `brackets.scm`, nvim-treesitter). Each pattern captures an
; opening token as @open and its matching closing token as @close among the
; siblings of a single parent node, so blocks, arrays, parameter lists, and
; grouped expressions all pair correctly.
;
; Mirrors the structural bracket pairs declared for the VS Code client in
; `language-configuration.json` ("brackets": [], i.e. {} [] ()). String quotes
; are intentionally omitted here (they are an auto-closing pair, not a
; structural bracket).

("(" @open ")" @close)
("[" @open "]" @close)
("{" @open "}" @close)
