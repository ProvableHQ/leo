[
  "let" "const" "constant" "fn" "final" "struct" "record"
  "program" "import" "mapping" "storage" "interface" "constructor"
  "if" "else" "for" "in" "return"
  "public" "private" "as" "self" "block" "network"
  "assert" "assert_eq" "assert_neq"
  "Final" "Fn"
] @keyword

"aleo" @keyword

(boolean_literal) @constant.builtin
(none_literal) @constant.builtin
(special_path) @constant.builtin

(primitive_type) @type.builtin

(function_definition name: (identifier) @function)
(final_function_definition name: (identifier) @function)
(constructor_definition "constructor" @function)

(call_expression function: (_) @function.call)
(method_call_expression method: (_) @function.method.call)

(struct_definition name: (identifier) @type)
(record_definition name: (identifier) @type)
(interface_declaration name: (identifier) @type)

(path_type) @type
(locator_type) @type

(program_declaration name: (program_id) @namespace)
(import_declaration name: (program_id) @namespace)

(annotation name: (_) @attribute)
"@" @attribute

(field_expression field: (_) @property)
(struct_field name: (identifier) @property)
(struct_field_init name: (identifier) @property)

(parameter name: (identifier) @variable.parameter)

(identifier_pattern) @variable

(integer_literal) @number
(field_literal) @number
(group_literal) @number
(scalar_literal) @number
(string_literal) @string
(address_literal) @string.special

(line_comment) @comment
(block_comment) @comment

[
  "+" "-" "*" "/" "%" "**"
  "==" "!=" "<" "<=" ">" ">="
  "&&" "||" "!"
  "&" "|" "^" "<<" ">>"
  "=" "+=" "-=" "*=" "/=" "%=" "**="
  "&&=" "||=" "&=" "|=" "^=" "<<=" ">>="
  "as" "?" ".." "..="
] @operator

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
["," "." ";" ":" "::" "->" "=>" "@"] @punctuation.delimiter
