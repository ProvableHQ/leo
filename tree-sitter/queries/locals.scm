(block) @local.scope
(function_definition) @local.scope
(final_function_definition) @local.scope
(constructor_definition) @local.scope
(for_statement) @local.scope

(let_statement pattern: (identifier_pattern (identifier) @local.definition))
(const_statement name: (identifier) @local.definition)
(parameter name: (identifier) @local.definition)
(for_statement name: (identifier) @local.definition)

(identifier) @local.reference
