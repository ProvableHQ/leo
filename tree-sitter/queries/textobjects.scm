(function_definition) @function.outer
(function_definition body: (block) @function.inner)
(final_function_definition) @function.outer
(final_function_definition body: (block) @function.inner)
(constructor_definition) @function.outer
(constructor_definition body: (block) @function.inner)

(struct_definition) @class.outer
(struct_definition body: (struct_field_list) @class.inner)
(record_definition) @class.outer
(record_definition body: (struct_field_list) @class.inner)

(parameter) @parameter.outer
(argument_list (_) @parameter.inner)

(block) @block.outer

(line_comment) @comment.outer
(block_comment) @comment.outer

(if_statement) @conditional.outer
(if_statement consequence: (block) @conditional.inner)

(for_statement) @loop.outer
(for_statement body: (block) @loop.inner)
