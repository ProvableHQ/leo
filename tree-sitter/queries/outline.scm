; Outline / symbol queries for Leo.
;
; Consumed by editors that build their symbol outline from tree-sitter
; (e.g. Zed's `outline.scm`). Each top-level declaration is captured as an
; @item whose label is built from the keyword(s) (@context) and the
; declaration name (@name). This drives the outline panel with or without the
; language server, so it must cover every program-level declaration form.

; Program + imports.
(program_declaration
  "program" @context
  name: (program_id) @name) @item

(import_declaration
  "import" @context
  name: (program_id) @name) @item

; Functions (plain / final / view) — show keyword(s), name, and signature.
(function_definition
  "fn" @context
  name: (identifier) @name
  parameters: (parameter_list) @context) @item

(final_function_definition
  "final" @context
  "fn" @context
  name: (identifier) @name
  parameters: (parameter_list) @context) @item

(view_function_definition
  "view" @context
  "fn" @context
  name: (identifier) @name
  parameters: (parameter_list) @context) @item

; The constructor has no name; label it with its keyword.
(constructor_definition
  "constructor" @name) @item

; Types.
(struct_definition
  "struct" @context
  name: (identifier) @name) @item

(record_definition
  "record" @context
  name: (identifier) @name) @item

(interface_declaration
  "interface" @context
  name: (identifier) @name) @item

; Program state.
(mapping_definition
  "mapping" @context
  name: (identifier) @name) @item

(storage_definition
  "storage" @context
  name: (identifier) @name) @item

(const_declaration
  "const" @context
  name: (identifier) @name) @item

; Interface members (prototypes) — nest under their interface in the outline.
(function_prototype
  "fn" @context
  name: (identifier) @name
  parameters: (parameter_list) @context) @item

(record_prototype
  "record" @context
  name: (identifier) @name) @item
