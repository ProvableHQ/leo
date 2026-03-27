const PREC = {
  ternary: 1,
  or: 2,
  and: 3,
  compare_eq: 4,
  compare_ord: 5,
  bitor: 6,
  bitxor: 7,
  bitand: 8,
  shift: 9,
  add: 10,
  multiply: 11,
  power: 12,
  cast: 13,
  unary: 14,
  call: 15,
};

const ALL_KEYWORDS = [
  'true',
  'false',
  'none',
  'address',
  'bool',
  'field',
  'group',
  'scalar',
  'signature',
  'string',
  'record',
  'i8',
  'i16',
  'i32',
  'i64',
  'i128',
  'u8',
  'u16',
  'u32',
  'u64',
  'u128',
  'if',
  'else',
  'for',
  'in',
  'return',
  'let',
  'const',
  'constant',
  'final',
  'fn',
  'struct',
  'constructor',
  'interface',
  'program',
  'import',
  'mapping',
  'storage',
  'network',
  'aleo',
  'script',
  'block',
  'public',
  'private',
  'as',
  'self',
  'assert',
  'assert_eq',
  'assert_neq',
  'Final',
  'Fn',
];

function commaSep1(rule) {
  return seq(rule, repeat(seq(',', rule)), optional(','));
}

module.exports = grammar({
  name: 'leo',

  extras: $ => [
    /\s/,
    $.line_comment,
    $.block_comment,
  ],

  word: $ => $.identifier,

  supertypes: $ => [
    $._expression,
    $._statement,
    $._type,
    $._item,
    $._pattern,
    $._literal,
  ],

  conflicts: $ => [
    [$.struct_expression, $.path_expression],
    [$.struct_expression, $._expression],
    [$.struct_expression, $._expression_no_struct],
    [$.struct_locator_expression, $.path_locator_expression],
    [$.struct_locator_expression, $._expression],
    [$.struct_locator_expression, $._expression_no_struct],
    [$.path_type, $.path_expression],
    [$.locator_type, $.program_ref_expression],
    [$.program_id, $.path_expression],
    [$.path_locator_expression, $.program_ref_expression],
    [$.path_locator_expression, $.locator_type],
    [$.path_locator_expression, $.program_ref_expression, $.locator_type],
    [$.return_type, $.tuple_type],
    [$.return_type_item, $.tuple_type],
    [$.optional_type, $.mapping_type],
    [$.tuple_type, $.tuple_expression],
    [$._type, $.parenthesized_expression],
    [$.call_expression, $.method_call_expression],
  ],

  rules: {
    source_file: $ => repeat($._file_item),

    _item: $ => choice(
      $.import_declaration,
      $.program_declaration,
      $.interface_declaration,
      $.struct_definition,
      $.record_definition,
      $.mapping_definition,
      $.storage_definition,
      $.const_declaration,
      $.function_definition,
      $.final_function_definition,
      $.constructor_definition,
    ),

    _file_item: $ => choice(
      $.import_declaration,
      $.program_declaration,
      $.interface_declaration,
      $.struct_definition,
      $.function_definition,
      $.final_function_definition,
      $.const_declaration,
    ),

    _program_item: $ => choice(
      $.struct_definition,
      $.record_definition,
      $.mapping_definition,
      $.storage_definition,
      $.const_declaration,
      $.function_definition,
      $.final_function_definition,
      $.constructor_definition,
      $.interface_declaration,
    ),

    import_declaration: $ => seq(
      'import',
      field('name', $.program_id),
      ';',
    ),

    program_id: $ => seq(
      field('name', $.identifier),
      '.',
      'aleo',
    ),

    program_declaration: $ => seq(
      'program',
      field('name', $.program_id),
      optional(seq(':', field('parents', $.parent_list))),
      '{',
      repeat($._program_item),
      '}',
    ),

    parent_list: $ => seq(
      $._type,
      repeat(seq('+', $._type)),
    ),

    interface_declaration: $ => seq(
      'interface',
      field('name', $.identifier),
      optional(seq(':', field('parents', $.parent_list))),
      '{',
      repeat($.interface_item),
      '}',
    ),

    interface_item: $ => choice(
      $.function_prototype,
      $.record_prototype,
      $.mapping_definition,
      $.storage_definition,
    ),

    function_prototype: $ => seq(
      'fn',
      field('name', $.identifier),
      optional(field('const_parameters', $.const_param_list)),
      field('parameters', $.parameter_list),
      optional(seq(token(prec(1, '->')), field('return_type', $.return_type))),
      ';',
    ),

    record_prototype: $ => seq(
      'record',
      field('name', $.identifier),
      ';',
    ),

    struct_definition: $ => seq(
      'struct',
      field('name', $.identifier),
      optional(field('const_parameters', $.const_param_list)),
      field('body', $.struct_field_list),
    ),

    record_definition: $ => seq(
      'record',
      field('name', $.identifier),
      optional(field('const_parameters', $.const_param_list)),
      field('body', $.struct_field_list),
    ),

    struct_field_list: $ => seq(
      '{',
      optional(commaSep1($.struct_field)),
      '}',
    ),

    struct_field: $ => seq(
      optional($.visibility),
      field('name', $.identifier),
      ':',
      field('type', $._type),
    ),

    visibility: _ => choice(
      'public',
      'private',
      'constant',
    ),

    mapping_definition: $ => seq(
      'mapping',
      field('name', $.identifier),
      ':',
      field('key', $._type),
      token(prec(1, '=>')),
      field('value', $._type),
      ';',
    ),

    storage_definition: $ => seq(
      'storage',
      field('name', $.identifier),
      ':',
      field('type', $._type),
      ';',
    ),

    const_declaration: $ => seq(
      'const',
      field('name', $.identifier),
      ':',
      field('type', $._type),
      '=',
      field('value', $._expression),
      ';',
    ),

    function_definition: $ => seq(
      repeat($.annotation),
      'fn',
      field('name', $.identifier),
      optional(field('const_parameters', $.const_param_list)),
      field('parameters', $.parameter_list),
      optional(seq(token(prec(1, '->')), field('return_type', $.return_type))),
      field('body', $.block),
    ),

    final_function_definition: $ => seq(
      repeat($.annotation),
      'final',
      'fn',
      field('name', $.identifier),
      optional(field('const_parameters', $.const_param_list)),
      field('parameters', $.parameter_list),
      optional(seq(token(prec(1, '->')), field('return_type', $.return_type))),
      field('body', $.block),
    ),

    constructor_definition: $ => seq(
      repeat($.annotation),
      'constructor',
      field('parameters', $.parameter_list),
      field('body', $.block),
    ),

    const_param_list: $ => seq(
      token(prec(1, '::')),
      '[',
      commaSep1($.const_param),
      ']',
    ),

    const_param: $ => seq(
      field('name', $.identifier),
      ':',
      field('type', $._type),
    ),

    parameter_list: $ => seq(
      '(',
      optional(commaSep1($.parameter)),
      ')',
    ),

    parameter: $ => seq(
      optional($.visibility),
      field('name', $.identifier),
      ':',
      field('type', $._type),
    ),

    return_type: $ => choice(
      $.return_type_item,
      seq('(', optional(commaSep1($.return_type_item)), ')'),
    ),

    return_type_item: $ => seq(
      optional($.visibility),
      $._type,
    ),

    annotation: $ => seq(
      '@',
      field('name', $._annotation_name),
      optional(seq('(', commaSep1($.annotation_pair), ')')),
    ),

    annotation_pair: $ => seq(
      field('key', $._annotation_key),
      '=',
      field('value', $.string_literal),
    ),

    _annotation_name: $ => choice(
      $.identifier,
      ...ALL_KEYWORDS,
    ),

    _annotation_key: $ => choice(
      $.identifier,
      'address',
      'mapping',
    ),

    _statement: $ => choice(
      $.let_statement,
      $.const_statement,
      $.return_statement,
      $.if_statement,
      $.for_statement,
      $.assert_statement,
      $.assert_eq_statement,
      $.assert_neq_statement,
      $.block,
      $.compound_assignment_statement,
      $.assignment_statement,
      $.expression_statement,
    ),

    let_statement: $ => seq(
      'let',
      field('pattern', $._pattern),
      optional(seq(':', field('type', $._type))),
      '=',
      field('value', $._expression),
      ';',
    ),

    const_statement: $ => seq(
      'const',
      field('name', $.identifier),
      ':',
      field('type', $._type),
      '=',
      field('value', $._expression),
      ';',
    ),

    return_statement: $ => seq(
      'return',
      optional(field('value', $._expression)),
      ';',
    ),

    expression_statement: $ => seq(
      field('expression', $._expression),
      ';',
    ),

    assignment_statement: $ => seq(
      field('left', $._expression),
      '=',
      field('right', $._expression),
      ';',
    ),

    compound_assignment_statement: $ => seq(
      field('left', $._expression),
      field('operator', $.compound_assign_op),
      field('right', $._expression),
      ';',
    ),

    compound_assign_op: _ => choice(
      token(prec(1, '**=')),
      token(prec(1, '&&=')),
      token(prec(1, '||=')),
      token(prec(1, '<<=')),
      token(prec(1, '>>=')),
      '+=',
      '-=',
      '*=',
      '/=',
      '%=',
      '&=',
      '|=',
      '^=',
    ),

    if_statement: $ => seq(
      'if',
      field('condition', $._expression_no_struct),
      field('consequence', $.block),
      optional(seq('else', field('alternative', choice($.if_statement, $.block)))),
    ),

    for_statement: $ => seq(
      'for',
      field('name', $.identifier),
      optional(seq(':', field('type', $._type))),
      'in',
      field('start', $._expression_no_struct),
      field('operator', choice(token(prec(1, '..=')), token('..'))),
      field('end', $._expression_no_struct),
      field('body', $.block),
    ),

    assert_statement: $ => seq(
      'assert',
      '(',
      field('condition', $._expression),
      ')',
      ';',
    ),

    assert_eq_statement: $ => seq(
      'assert_eq',
      '(',
      field('left', $._expression),
      ',',
      field('right', $._expression),
      ')',
      ';',
    ),

    assert_neq_statement: $ => seq(
      'assert_neq',
      '(',
      field('left', $._expression),
      ',',
      field('right', $._expression),
      ')',
      ';',
    ),

    block: $ => seq(
      '{',
      repeat($._statement),
      '}',
    ),

    _pattern: $ => choice(
      $.identifier_pattern,
      $.tuple_pattern,
      $.wildcard_pattern,
    ),

    identifier_pattern: $ => seq($.identifier),

    tuple_pattern: $ => seq(
      '(',
      $._pattern,
      repeat(seq(',', $._pattern)),
      optional(','),
      ')',
    ),

    wildcard_pattern: $ => $.underscore,

    _expression: $ => choice(
      $.binary_expression,
      $.unary_expression,
      $.cast_expression,
      $.ternary_expression,
      $.call_expression,
      $.method_call_expression,
      $.field_expression,
      $.tuple_access_expression,
      $.index_expression,
      $.path_expression,
      $.struct_expression,
      $.struct_locator_expression,
      $.path_locator_expression,
      $.program_ref_expression,
      $.self_expression,
      $.block_keyword_expression,
      $.network_keyword_expression,
      $.parenthesized_expression,
      $.tuple_expression,
      $.array_expression,
      $.repeat_expression,
      $.final_expression,
      $._literal,
    ),

    _expression_no_struct: $ => choice(
      $.binary_expression,
      $.unary_expression,
      $.cast_expression,
      $.ternary_expression,
      $.call_expression,
      $.method_call_expression,
      $.field_expression,
      $.tuple_access_expression,
      $.index_expression,
      $.path_expression,
      $.path_locator_expression,
      $.program_ref_expression,
      $.self_expression,
      $.block_keyword_expression,
      $.network_keyword_expression,
      $.parenthesized_expression,
      $.tuple_expression,
      $.array_expression,
      $.repeat_expression,
      $.final_expression,
      $._literal,
    ),

    binary_expression: $ => {
      const leftTable = [
        [PREC.or, token(prec(1, '||'))],
        [PREC.and, token(prec(1, '&&'))],
        [PREC.bitor, '|'],
        [PREC.bitxor, '^'],
        [PREC.bitand, '&'],
        [PREC.shift, choice(token(prec(1, '<<')), token(prec(1, '>>')))],
        [PREC.add, choice('+', '-')],
        [PREC.multiply, choice('*', '/', '%')],
      ];

      return choice(
        ...leftTable.map(([precedence, operator]) => prec.left(precedence, seq(
          field('left', $._expression),
          field('operator', operator),
          field('right', $._expression),
        ))),
        prec.right(PREC.power, seq(
          field('left', $._expression),
          field('operator', token(prec(1, '**'))),
          field('right', $._expression),
        )),
        prec.left(PREC.compare_eq, seq(
          field('left', $._expression),
          field('operator', choice(token(prec(1, '==')), token(prec(1, '!=')))),
          field('right', $._expression),
        )),
        prec.left(PREC.compare_ord, seq(
          field('left', $._expression),
          field('operator', choice('<', token(prec(1, '<=')), '>', token(prec(1, '>=')))),
          field('right', $._expression),
        )),
      );
    },

    unary_expression: $ => prec.right(PREC.unary, seq(
      field('operator', choice('!', '-')),
      field('operand', $._expression),
    )),

    cast_expression: $ => prec.left(PREC.cast, seq(
      field('value', $._expression),
      'as',
      field('type', $.primitive_type),
    )),

    ternary_expression: $ => prec.right(PREC.ternary, seq(
      field('condition', $._expression),
      '?',
      field('consequence', $._expression),
      ':',
      field('alternative', $._expression),
    )),

    call_expression: $ => prec.left(PREC.call, seq(
      field('function', $._expression),
      field('arguments', $.argument_list),
    )),

    method_call_expression: $ => prec.left(PREC.call + 1, seq(
      field('value', $._expression),
      '.',
      field('method', $._field_name),
      field('arguments', $.argument_list),
    )),

    argument_list: $ => seq(
      '(',
      optional(commaSep1($._expression)),
      ')',
    ),

    field_expression: $ => prec.left(PREC.call, seq(
      field('value', $._expression),
      '.',
      field('field', $._field_name),
    )),

    tuple_access_expression: $ => prec.left(PREC.call + 1, seq(
      field('value', $._expression),
      '.',
      field('field', $.integer_literal),
    )),

    index_expression: $ => prec.left(PREC.call, seq(
      field('value', $._expression),
      '[',
      field('index', $._expression),
      ']',
    )),

    path_expression: $ => choice(
      $.special_path,
      seq(
        $.identifier,
        repeat(seq(token(prec(1, '::')), $.identifier)),
        optional($.const_arg_list),
      ),
    ),

    struct_expression: $ => seq(
      field('name', $.path_expression),
      field('body', $.struct_field_init_list),
    ),

    struct_locator_expression: $ => seq(
      field('name', $.path_locator_expression),
      field('body', $.struct_field_init_list),
    ),

    path_locator_expression: $ => seq(
      field('program', $.program_id),
      token(prec(1, '::')),
      field('member', $.identifier),
      optional($.const_arg_list),
    ),

    program_ref_expression: $ => $.program_id,

    struct_field_init_list: $ => seq(
      '{',
      optional(commaSep1($.struct_field_init)),
      '}',
    ),

    struct_field_init: $ => seq(
      field('name', $.identifier),
      optional(seq(':', field('value', $._expression))),
    ),

    const_arg_list: $ => choice(
      $.const_arg_list_bracket,
      $.const_arg_list_angle,
    ),

    const_arg_list_bracket: $ => seq(
      token(prec(1, '::')),
      '[',
      commaSep1($.const_arg),
      ']',
    ),

    const_arg: $ => choice(
      $._type,
      $._expression,
    ),

    const_arg_list_angle: $ => seq(
      token(prec(1, '::')),
      '<',
      commaSep1($._simple_const_arg),
      '>',
    ),

    _simple_const_arg: $ => choice(
      $.identifier,
      $.integer_literal,
    ),

    self_expression: _ => 'self',

    block_keyword_expression: _ => 'block',

    network_keyword_expression: _ => 'network',

    parenthesized_expression: $ => seq(
      '(',
      $._expression,
      ')',
    ),

    tuple_expression: $ => choice(
      seq('(', ')'),
      seq('(', $._expression, ',', commaSep1($._expression), ')'),
    ),

    array_expression: $ => seq(
      '[',
      optional(commaSep1($._expression)),
      ']',
    ),

    repeat_expression: $ => seq(
      '[',
      field('value', $._expression),
      ';',
      field('count', $._expression),
      ']',
    ),

    final_expression: $ => seq(
      'final',
      field('body', $.block),
    ),

    _literal: $ => choice(
      $.integer_literal,
      $.field_literal,
      $.group_literal,
      $.scalar_literal,
      $.string_literal,
      $.address_literal,
      $.boolean_literal,
      $.none_literal,
    ),

    integer_literal: _ => token(choice(
      /[0-9][0-9A-Za-z_]*(u8|u16|u32|u64|u128|i8|i16|i32|i64|i128)?/,
      /0x[0-9A-Za-z_]+(u8|u16|u32|u64|u128|i8|i16|i32|i64|i128)?/,
      /0o[0-9A-Za-z_]+(u8|u16|u32|u64|u128|i8|i16|i32|i64|i128)?/,
      /0b[0-9A-Za-z_]+(u8|u16|u32|u64|u128|i8|i16|i32|i64|i128)?/,
    )),

    field_literal: _ => token(prec(2, /[0-9][0-9_]*field/)),

    group_literal: _ => token(prec(2, /[0-9][0-9_]*group/)),

    scalar_literal: _ => token(prec(2, /[0-9][0-9_]*scalar/)),

    string_literal: _ => token(/"[^"]*"/),

    address_literal: _ => token(/aleo1[a-z0-9]+/),

    boolean_literal: _ => choice('true', 'false'),

    none_literal: _ => 'none',

    _type: $ => choice(
      $.primitive_type,
      $.path_type,
      $.locator_type,
      $.array_type,
      $.vector_type,
      $.tuple_type,
      $.optional_type,
      $.final_type,
      $.mapping_type,
    ),

    primitive_type: _ => choice(
      'address',
      'bool',
      'field',
      'group',
      'scalar',
      'signature',
      'string',
      'i8',
      'i16',
      'i32',
      'i64',
      'i128',
      'u8',
      'u16',
      'u32',
      'u64',
      'u128',
    ),

    path_type: $ => seq(
      $.identifier,
      repeat(seq(token(prec(1, '::')), $.identifier)),
      optional($.const_arg_list),
    ),

    locator_type: $ => seq(
      $.program_id,
      optional(seq(token(prec(1, '::')), $.identifier)),
      optional($.const_arg_list),
    ),

    array_type: $ => seq(
      '[',
      field('element', $._type),
      ';',
      field('length', $._expression),
      ']',
    ),

    vector_type: $ => seq(
      '[',
      field('element', $._type),
      ']',
    ),

    tuple_type: $ => choice(
      seq('(', ')'),
      seq('(', $._type, repeat(seq(',', $._type)), optional(','), ')'),
    ),

    optional_type: $ => prec.right(seq(
      field('type', $._type),
      '?',
    )),

    final_type: $ => choice(
      'Final',
      seq(
        'Final',
        '<',
        'Fn',
        '(',
        optional(commaSep1($._type)),
        ')',
        optional(seq(token(prec(1, '->')), field('return_type', $._type))),
        '>',
      ),
    ),

    mapping_type: $ => seq(
      'mapping',
      field('key', $._type),
      token(prec(1, '=>')),
      field('value', $._type),
    ),

    line_comment: _ => token(/\/\/[^\n]*/),

    block_comment: _ => token(seq(
      '/*',
      /[^*]*\*+([^/*][^*]*\*+)*/,
      '/',
    )),

    identifier: _ => /[a-zA-Z][a-zA-Z0-9_]*|_[a-zA-Z][a-zA-Z0-9_]*/,

    underscore: _ => '_',

    special_path: _ => token(prec(2, choice(
      /group::[a-zA-Z][a-zA-Z0-9_]*/,
      /signature::[a-zA-Z][a-zA-Z0-9_]*/,
      /Future::[a-zA-Z][a-zA-Z0-9_]*/,
    ))),

    _field_name: $ => choice(
      $.identifier,
      ...ALL_KEYWORDS,
    ),
  },
});
