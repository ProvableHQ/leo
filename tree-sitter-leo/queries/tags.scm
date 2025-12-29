; Tree-sitter tags queries for Leo
; Used for code navigation (go to definition, etc.)

; Functions
(function_declaration
  name: (identifier) @name) @definition.function

(constructor_declaration) @definition.function

; Structs and Records
(struct_declaration
  name: (identifier) @name) @definition.class

(record_declaration
  name: (identifier) @name) @definition.class

; Mappings
(mapping_declaration
  name: (identifier) @name) @definition.variable

; Storage
(storage_declaration
  name: (identifier) @name) @definition.variable

; Global Constants
(global_const
  name: (identifier) @name) @definition.constant

; Calls
(call_expression
  function: (identifier) @name) @reference.call

(associated_function_call
  type: (path) @name) @reference.call

(method_call
  method: (identifier) @name) @reference.call
