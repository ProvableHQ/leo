; Tree-sitter locals queries for Leo
; Defines scopes and variable references

; Scopes
(function_declaration) @local.scope
(block) @local.scope
(for_statement) @local.scope

; Definitions
(definition_statement
  name: (identifier) @local.definition)

(const_statement
  name: (identifier) @local.definition)

(parameter
  name: (identifier) @local.definition)

(for_statement
  variable: (identifier) @local.definition)

; References
(identifier) @local.reference
