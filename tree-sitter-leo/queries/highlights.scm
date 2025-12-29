; Tree-sitter syntax highlighting queries for Leo
; Compatible with Neovim, Helix, and other tree-sitter based editors

; ============================================
; Comments
; ============================================
(line_comment) @comment.line
(block_comment) @comment.block

; Fallback for editors that don't support specific comment types
(line_comment) @comment
(block_comment) @comment

; ============================================
; Keywords - Structure
; ============================================
[
  "program"
  "import"
] @keyword.control.import

[
  "struct"
  "record"
] @keyword.type

[
  "mapping"
  "storage"
] @keyword.storage

; ============================================
; Keywords - Functions
; ============================================
[
  "function"
  "transition"
  "inline"
  "script"
  "constructor"
] @keyword.function

"async" @keyword.coroutine

; ============================================
; Keywords - Variables
; ============================================
"let" @keyword.storage.type
"const" @keyword.storage.modifier

; ============================================
; Keywords - Control Flow
; ============================================
[
  "if"
  "else"
] @keyword.conditional

[
  "for"
  "in"
] @keyword.repeat

"return" @keyword.return

; ============================================
; Keywords - Operators
; ============================================
"as" @keyword.operator

; ============================================
; Keywords - Visibility/Modifiers
; ============================================
[
  "public"
  "private"
  "constant"
] @keyword.modifier

; Fallback for @type.qualifier
[
  "public"
  "private"
  "constant"
] @type.qualifier

; ============================================
; Built-in Functions
; ============================================
[
  "assert"
  "assert_eq"
  "assert_neq"
] @function.builtin

; ============================================
; Types - Built-in
; ============================================
(primitive_type) @type.builtin

(future_type) @type.builtin

[
  "Future"
  "Fn"
] @type.builtin

; Type suffix on literals (e.g., the "u32" in "42u32")
(type_suffix) @type.builtin

; ============================================
; Types - User-defined
; ============================================
; Struct/Record definitions
(struct_declaration
  name: (identifier) @type.definition)

(record_declaration
  name: (identifier) @type.definition)

; Type references in type position
(array_type
  element: (identifier) @type)

(tuple_type
  element: (identifier) @type)

(optional_type
  (identifier) @type)

(parameter
  type: (identifier) @type)

(struct_member
  type: (identifier) @type)

(definition_statement
  type: (identifier) @type)

(const_statement
  type: (identifier) @type)

(global_const
  type: (identifier) @type)

(mapping_declaration
  key_type: (identifier) @type)

(mapping_declaration
  value_type: (identifier) @type)

(storage_declaration
  type: (identifier) @type)

(const_parameter
  type: (identifier) @type)

(for_statement
  type: (identifier) @type)

; Paths used as types (e.g., Foo::Bar)
(path) @type

; ============================================
; Literals - Numbers
; ============================================
(integer_literal) @number

; Typed literals (the whole thing, e.g., "42u32")
(typed_literal
  (integer_literal) @number)

; ============================================
; Literals - Strings
; ============================================
(string_literal) @string

; ============================================
; Literals - Boolean and Special
; ============================================
(boolean_literal) @boolean
(boolean_literal) @constant.builtin

(none_literal) @constant.builtin

; ============================================
; Literals - Address
; ============================================
(address_literal) @string.special

; Address-like identifiers (aleo1...)
((identifier) @string.special
  (#match? @string.special "^aleo1"))

; ============================================
; Identifiers - Namespaces
; ============================================
(program_id) @namespace
(program_id) @module

(locator) @namespace

; ============================================
; Identifiers - Functions
; ============================================
; Function declarations
(function_declaration
  name: (identifier) @function)

(constructor_declaration
  "constructor" @function)

; Function calls
(call_expression
  function: (identifier) @function.call)

(call_expression
  function: (locator) @function.call)

; Associated function calls (e.g., Mapping::get)
(associated_function_call
  type: (path) @type)

; Extract the function name from path in associated calls
((path) @function.call
  (#match? @function.call "::"))

; Method calls
(method_call
  method: (identifier) @function.method.call)

(method_call
  method: (identifier) @function.method)

; ============================================
; Identifiers - Parameters
; ============================================
(parameter
  name: (identifier) @variable.parameter)

(const_parameter
  name: (identifier) @variable.parameter)

; ============================================
; Identifiers - Properties/Fields
; ============================================
; Struct member declarations
(struct_member
  name: (identifier) @property.definition)

(struct_member
  name: (identifier) @field)

; Struct member initializers
(struct_member_initializer
  name: (identifier) @property)

(struct_member_initializer
  name: (identifier) @field)

; Member access
(member_access
  member: (identifier) @property)

(member_access
  member: (identifier) @field)

; Tuple access index
(tuple_access
  index: (integer_literal) @number)

; ============================================
; Identifiers - Variables
; ============================================
; Variable definitions
(definition_statement
  name: (identifier) @variable)

; For loop variable
(for_statement
  variable: (identifier) @variable)

; ============================================
; Identifiers - Constants
; ============================================
(const_statement
  name: (identifier) @constant)

(global_const
  name: (identifier) @constant)

; ============================================
; Identifiers - Mappings/Storage
; ============================================
(mapping_declaration
  name: (identifier) @variable)

(storage_declaration
  name: (identifier) @variable)

; ============================================
; Annotations
; ============================================
(annotation
  "@" @attribute.builtin)

(annotation
  "@" @punctuation.special)

(annotation
  name: (identifier) @attribute)

(annotation_member
  key: (identifier) @property)

(annotation_member
  key: "mapping" @property)

(annotation_member
  key: "address" @property)

; ============================================
; Special Access (self, block, network)
; ============================================
(special_access
  object: "self" @variable.builtin)

(special_access
  object: "block" @variable.builtin)

(special_access
  object: "network" @variable.builtin)

(special_access
  member: (identifier) @property)

(special_access
  member: "address" @property)

; Standalone self, block, network keywords
["self" "block" "network"] @variable.builtin

; ============================================
; Operators - Arithmetic
; ============================================
[
  "+"
  "-"
  "*"
  "/"
  "%"
  "**"
] @operator

; ============================================
; Operators - Bitwise
; ============================================
[
  "<<"
  ">>"
  "&"
  "|"
  "^"
] @operator

; ============================================
; Operators - Logical
; ============================================
[
  "!"
  "&&"
  "||"
] @operator

; ============================================
; Operators - Comparison
; ============================================
[
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
] @operator

; ============================================
; Operators - Ternary
; ============================================
(ternary_expression
  "?" @conditional.ternary)

(ternary_expression
  ":" @conditional.ternary)

; Fallback
(ternary_expression
  "?" @operator)

(ternary_expression
  ":" @operator)

; ============================================
; Operators - Assignment
; ============================================
(assignment_operator) @operator

"=" @operator

; ============================================
; Punctuation - Brackets
; ============================================
[
  "("
  ")"
] @punctuation.bracket

[
  "["
  "]"
] @punctuation.bracket

[
  "{"
  "}"
] @punctuation.bracket

; ============================================
; Punctuation - Delimiters
; ============================================
"," @punctuation.delimiter
";" @punctuation.delimiter
"." @punctuation.delimiter
"::" @punctuation.delimiter
"->" @punctuation.delimiter
"=>" @punctuation.delimiter
".." @punctuation.delimiter
":" @punctuation.delimiter

; ============================================
; Error nodes (for editors that support it)
; ============================================
(ERROR) @error
