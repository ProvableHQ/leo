; Tree-sitter syntax highlighting queries for Leo

; ============================================
; Comments
; ============================================
(line_comment) @comment
(block_comment) @comment

; ============================================
; Keywords
; ============================================
[
  "program"
  "import"
  "struct"
  "record"
  "mapping"
  "storage"
] @keyword

[
  "function"
  "transition"
  "inline"
  "script"
  "constructor"
  "async"
] @keyword.function

[
  "let"
  "const"
] @keyword

[
  "if"
  "else"
  "for"
  "in"
  "return"
] @keyword.control

[
  "as"
] @keyword.operator

[
  "public"
  "private"
  "constant"
] @keyword.modifier

[
  "assert"
  "assert_eq"
  "assert_neq"
] @function.builtin

; ============================================
; Types
; ============================================
(primitive_type) @type.builtin

(future_type) @type.builtin

[
  "Future"
  "Fn"
] @type.builtin

(struct_declaration
  name: (identifier) @type.definition)

(record_declaration
  name: (identifier) @type.definition)

(function_output) @type

; ============================================
; Literals
; ============================================
(integer_literal) @number
(typed_literal) @number
(type_suffix) @type.builtin

(string_literal) @string

(boolean_literal) @constant.builtin
(none_literal) @constant.builtin

(address_literal) @string.special

; ============================================
; Identifiers
; ============================================
(program_id) @namespace

(locator) @namespace

(path) @type

; Function names
(function_declaration
  name: (identifier) @function)

(constructor_declaration) @function

(call_expression
  function: (identifier) @function.call)

(call_expression
  function: (locator) @function.call)

(associated_function_call
  type: (path) @type)

(method_call
  method: (identifier) @function.method.call)

; Parameters
(parameter
  name: (identifier) @variable.parameter)

; Struct members
(struct_member
  name: (identifier) @property.definition)

(struct_member_initializer
  name: (identifier) @property)

(member_access
  member: (identifier) @property)

; Variables in definitions
(definition_statement
  name: (identifier) @variable)

(const_statement
  name: (identifier) @constant)

(global_const
  name: (identifier) @constant)

(mapping_declaration
  name: (identifier) @variable)

(storage_declaration
  name: (identifier) @variable)

; For loop variable
(for_statement
  variable: (identifier) @variable)

; Annotations
(annotation
  "@" @attribute
  name: (identifier) @attribute)

; ============================================
; Operators
; ============================================
[
  "+"
  "-"
  "*"
  "/"
  "%"
  "**"
  "<<"
  ">>"
  "&"
  "|"
  "^"
  "!"
  "&&"
  "||"
  "=="
  "!="
  "<"
  ">"
  "<="
  ">="
  "?"
  ":"
] @operator

(assignment_operator) @operator

; ============================================
; Punctuation
; ============================================
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

[
  ","
  ";"
  "."
  "::"
  "->"
  "=>"
  ".."
] @punctuation.delimiter

; ============================================
; Special Access
; ============================================
(special_access
  object: _ @variable.builtin)
