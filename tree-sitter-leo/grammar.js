/**
 * @file Tree-sitter grammar for the Leo programming language
 * @author Leo Contributors
 * @license GPL-3.0
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const PREC = {
  TERNARY: 1,
  OR: 2,
  AND: 3,
  BIT_OR: 4,
  BIT_XOR: 5,
  BIT_AND: 6,
  EQUALITY: 7,
  COMPARISON: 8,
  SHIFT: 9,
  ADDITIVE: 10,
  MULTIPLICATIVE: 11,
  POWER: 12,
  UNARY: 13,
  CAST: 14,
  POSTFIX: 15,
  CALL: 16,
};

module.exports = grammar({
  name: 'leo',

  extras: $ => [
    /\s/,
    $.line_comment,
    $.block_comment,
  ],

  word: $ => $.identifier,

  conflicts: $ => [
    [$._expression_no_struct, $.struct_expression],
    [$._type, $._expression_no_struct],
    [$.optional_type, $._expression_no_struct],
    [$.tuple_type, $.function_output],
  ],

  rules: {
    // Entry point
    source_file: $ => seq(
      repeat($.import_declaration),
      $.program_declaration,
    ),

    // ============================================
    // Comments
    // ============================================
    line_comment: _ => token(seq('//', /.*/)),

    block_comment: _ => token(seq(
      '/*',
      /[^*]*\*+([^/*][^*]*\*+)*/,
      '/',
    )),

    // ============================================
    // Identifiers and Literals
    // ============================================
    identifier: _ => /[a-zA-Z_][a-zA-Z0-9_]*/,

    // Path like Foo::bar::baz
    path: $ => prec.left(seq(
      $.identifier,
      repeat1(seq('::', $.identifier)),
    )),

    // Program ID like my_program.aleo
    program_id: _ => /[a-zA-Z_][a-zA-Z0-9_]*\.aleo/,

    // Locator like my_program.aleo/my_function
    locator: _ => /[a-zA-Z_][a-zA-Z0-9_]*\.aleo\/[a-zA-Z_][a-zA-Z0-9_]*/,

    // Address literal - must have at least some characters after aleo1 to be an address
    // Address literals in Leo are exactly 63 characters (aleo1 + 58 chars)
    address_literal: _ => /aleo1[a-z0-9]+/,

    // Integer literal (decimal, hex, octal, binary)
    integer_literal: _ => choice(
      /0x[0-9a-fA-F_]+/,
      /0o[0-7_]+/,
      /0b[01_]+/,
      /[0-9][0-9_]*/,
    ),

    // String literal
    string_literal: _ => /"[^"]*"/,

    // Boolean literals
    boolean_literal: _ => choice('true', 'false'),

    // None literal
    none_literal: _ => 'none',

    // ============================================
    // Types
    // ============================================
    _type: $ => choice(
      $.primitive_type,
      $.array_type,
      $.tuple_type,
      $.unit_type,
      $.optional_type,
      $.future_type,
      $.identifier,
      $.locator,
      $.path,
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

    array_type: $ => seq(
      '[',
      field('element', $._type),
      ';',
      field('size', $._expression),
      ']',
    ),

    tuple_type: $ => seq(
      '(',
      field('element', $._type),
      repeat1(seq(',', field('element', $._type))),
      optional(','),
      ')',
    ),

    unit_type: _ => seq('(', ')'),

    optional_type: $ => prec.left(seq(
      choice(
        $.primitive_type,
        $.array_type,
        $.tuple_type,
        $.future_type,
        $.identifier,
        $.locator,
        $.path,
      ),
      '?',
    )),

    future_type: $ => choice(
      'Future',
      seq('Future', '<', 'Fn', '(', optional($.type_list), ')', '>'),
    ),

    type_list: $ => seq(
      $._type,
      repeat(seq(',', $._type)),
      optional(','),
    ),

    // Vector type (for storage)
    vector_type: $ => seq(
      '[',
      field('element', $._type),
      ']',
    ),

    // ============================================
    // Expressions - We split into two categories:
    // 1. _expression: All expressions including struct initializers
    // 2. _expression_no_struct: Expressions without struct initializers (for if/for conditions)
    // ============================================
    
    // Full expression (used most places)
    _expression: $ => choice(
      $._expression_no_struct,
      $.struct_expression,
    ),
    
    // Expression without struct initializer (used in if/for conditions)
    _expression_no_struct: $ => choice(
      $.literal,
      $.identifier,
      $.path,
      $.locator,
      $.parenthesized_expression,
      $.tuple_expression,
      $.array_expression,
      $.repeat_expression,
      $.call_expression,
      $.associated_function_call,
      $.method_call,
      $.member_access,
      $.tuple_access,
      $.array_access,
      $.unary_expression,
      $.binary_expression,
      $.ternary_expression,
      $.cast_expression,
      $.special_access,
      $.async_block,
    ),

    literal: $ => choice(
      $.address_literal,  // Put address_literal first to prioritize it
      $.typed_literal,
      $.integer_literal,
      $.string_literal,
      $.boolean_literal,
      $.none_literal,
    ),

    // Typed literals like 5u32, 10field, etc.
    typed_literal: $ => seq(
      $.integer_literal,
      $.type_suffix,
    ),

    type_suffix: _ => choice(
      'field',
      'group',
      'scalar',
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

    parenthesized_expression: $ => seq(
      '(',
      $._expression,
      ')',
    ),

    tuple_expression: $ => seq(
      '(',
      $._expression,
      ',',
      repeat(seq($._expression, ',')),
      optional($._expression),
      ')',
    ),

    array_expression: $ => seq(
      '[',
      optional(seq(
        $._expression,
        repeat(seq(',', $._expression)),
        optional(','),
      )),
      ']',
    ),

    repeat_expression: $ => seq(
      '[',
      field('element', $._expression),
      ';',
      field('count', $._expression),
      ']',
    ),

    struct_expression: $ => seq(
      field('name', choice($.identifier, $.path, $.locator)),
      optional($.const_arguments),
      '{',
      optional($.struct_member_initializers),
      '}',
    ),

    struct_member_initializers: $ => seq(
      $.struct_member_initializer,
      repeat(seq(',', $.struct_member_initializer)),
      optional(','),
    ),

    struct_member_initializer: $ => choice(
      seq(
        field('name', $.identifier),
        ':',
        field('value', $._expression),
      ),
      // Shorthand: just the identifier (when name matches value)
      field('name', $.identifier),
    ),

    call_expression: $ => prec(PREC.CALL, seq(
      field('function', choice($.identifier, $.locator)),
      optional($.const_arguments),
      '(',
      optional($.argument_list),
      ')',
    )),

    associated_function_call: $ => prec(PREC.CALL, seq(
      field('type', $.path),
      optional($.const_arguments),
      '(',
      optional($.argument_list),
      ')',
    )),

    method_call: $ => prec.left(PREC.POSTFIX, seq(
      field('object', $._expression),
      '.',
      field('method', $.identifier),
      '(',
      optional($.argument_list),
      ')',
    )),

    argument_list: $ => seq(
      $._expression,
      repeat(seq(',', $._expression)),
      optional(','),
    ),

    const_arguments: $ => seq(
      '::',
      '[',
      optional($.const_argument_list),
      ']',
    ),

    const_argument_list: $ => seq(
      $._const_argument,
      repeat(seq(',', $._const_argument)),
      optional(','),
    ),

    _const_argument: $ => choice(
      $._expression,
      $._type,
    ),

    member_access: $ => prec.left(PREC.POSTFIX, seq(
      field('object', $._expression),
      '.',
      field('member', $.identifier),
    )),

    tuple_access: $ => prec.left(PREC.POSTFIX, seq(
      field('tuple', $._expression),
      '.',
      field('index', $.integer_literal),
    )),

    array_access: $ => prec.left(PREC.POSTFIX, seq(
      field('array', $._expression),
      '[',
      field('index', $._expression),
      ']',
    )),

    unary_expression: $ => prec.right(PREC.UNARY, seq(
      field('operator', choice('!', '-')),
      field('operand', $._expression),
    )),

    binary_expression: $ => choice(
      // Power (right associative)
      prec.right(PREC.POWER, seq(
        field('left', $._expression),
        field('operator', '**'),
        field('right', $._expression),
      )),
      // Multiplicative
      prec.left(PREC.MULTIPLICATIVE, seq(
        field('left', $._expression),
        field('operator', choice('*', '/', '%')),
        field('right', $._expression),
      )),
      // Additive
      prec.left(PREC.ADDITIVE, seq(
        field('left', $._expression),
        field('operator', choice('+', '-')),
        field('right', $._expression),
      )),
      // Shift
      prec.left(PREC.SHIFT, seq(
        field('left', $._expression),
        field('operator', choice('<<', '>>')),
        field('right', $._expression),
      )),
      // Bitwise AND
      prec.left(PREC.BIT_AND, seq(
        field('left', $._expression),
        field('operator', '&'),
        field('right', $._expression),
      )),
      // Bitwise XOR
      prec.left(PREC.BIT_XOR, seq(
        field('left', $._expression),
        field('operator', '^'),
        field('right', $._expression),
      )),
      // Bitwise OR
      prec.left(PREC.BIT_OR, seq(
        field('left', $._expression),
        field('operator', '|'),
        field('right', $._expression),
      )),
      // Comparison (not associative)
      prec.left(PREC.COMPARISON, seq(
        field('left', $._expression),
        field('operator', choice('<', '>', '<=', '>=')),
        field('right', $._expression),
      )),
      // Equality (not associative)
      prec.left(PREC.EQUALITY, seq(
        field('left', $._expression),
        field('operator', choice('==', '!=')),
        field('right', $._expression),
      )),
      // Logical AND
      prec.left(PREC.AND, seq(
        field('left', $._expression),
        field('operator', '&&'),
        field('right', $._expression),
      )),
      // Logical OR
      prec.left(PREC.OR, seq(
        field('left', $._expression),
        field('operator', '||'),
        field('right', $._expression),
      )),
    ),

    ternary_expression: $ => prec.right(PREC.TERNARY, seq(
      field('condition', $._expression),
      '?',
      field('consequence', $._expression),
      ':',
      field('alternative', $._expression),
    )),

    cast_expression: $ => prec.left(PREC.CAST, seq(
      field('value', $._expression),
      'as',
      field('type', $.primitive_type),
    )),

    special_access: $ => seq(
      field('object', choice('block', 'self', 'network')),
      '.',
      field('member', choice($.identifier, 'address')),
    ),

    async_block: $ => seq(
      'async',
      $.block,
    ),

    // ============================================
    // Statements
    // ============================================
    _statement: $ => choice(
      $.expression_statement,
      $.return_statement,
      $.definition_statement,
      $.const_statement,
      $.assignment_statement,
      $.conditional_statement,
      $.for_statement,
      $.assert_statement,
      $.assert_eq_statement,
      $.assert_neq_statement,
      $.block,
    ),

    block: $ => seq(
      '{',
      repeat($._statement),
      '}',
    ),

    expression_statement: $ => seq(
      $._expression,
      ';',
    ),

    return_statement: $ => seq(
      'return',
      optional($._expression),
      ';',
    ),

    definition_statement: $ => seq(
      'let',
      choice(
        // Single variable
        seq(
          field('name', $.identifier),
          optional(seq(':', field('type', $._type))),
        ),
        // Multiple variables (destructuring)
        seq(
          '(',
          field('name', $.identifier),
          repeat1(seq(',', field('name', $.identifier))),
          optional(','),
          ')',
          optional(seq(':', field('type', $._type))),
        ),
      ),
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

    assignment_statement: $ => seq(
      field('left', $._expression),
      field('operator', $.assignment_operator),
      field('right', $._expression),
      ';',
    ),

    assignment_operator: _ => choice(
      '=',
      '+=',
      '-=',
      '*=',
      '/=',
      '%=',
      '**=',
      '<<=',
      '>>=',
      '&=',
      '|=',
      '^=',
      '&&=',
      '||=',
    ),

    // Use _expression_no_struct for condition to avoid ambiguity with struct init
    conditional_statement: $ => prec.right(seq(
      'if',
      field('condition', $._expression_no_struct),
      field('consequence', $.block),
      optional(seq(
        'else',
        field('alternative', choice($.block, $.conditional_statement)),
      )),
    )),

    // Use _expression_no_struct for loop bounds to avoid ambiguity with struct init
    for_statement: $ => seq(
      'for',
      field('variable', $.identifier),
      optional(seq(':', field('type', $._type))),
      'in',
      field('start', $._expression_no_struct),
      '..',
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

    // ============================================
    // Program Structure
    // ============================================
    import_declaration: $ => seq(
      'import',
      $.program_id,
      ';',
    ),

    program_declaration: $ => seq(
      'program',
      $.program_id,
      '{',
      repeat($._program_item),
      '}',
    ),

    _program_item: $ => choice(
      $.function_declaration,
      $.struct_declaration,
      $.record_declaration,
      $.mapping_declaration,
      $.storage_declaration,
      $.global_const,
      $.constructor_declaration,
    ),

    // ============================================
    // Functions
    // ============================================
    annotation: $ => seq(
      '@',
      field('name', $.identifier),
      optional($.annotation_arguments),
    ),

    annotation_arguments: $ => seq(
      '(',
      optional(seq(
        $.annotation_member,
        repeat(seq(',', $.annotation_member)),
        optional(','),
      )),
      ')',
    ),

    annotation_member: $ => seq(
      field('key', choice($.identifier, 'mapping', 'address')),
      '=',
      field('value', $.string_literal),
    ),

    function_declaration: $ => seq(
      repeat($.annotation),
      optional('async'),
      field('kind', $.function_kind),
      field('name', $.identifier),
      optional($.const_parameters),
      $.parameter_list,
      optional(seq('->', $.function_output)),
      $.block,
    ),

    function_kind: _ => choice(
      'function',
      'transition',
      'inline',
      'script',
    ),

    const_parameters: $ => seq(
      '::',
      '[',
      optional($.const_parameter_list),
      ']',
    ),

    const_parameter_list: $ => seq(
      $.const_parameter,
      repeat(seq(',', $.const_parameter)),
      optional(','),
    ),

    const_parameter: $ => seq(
      field('name', $.identifier),
      ':',
      field('type', $._type),
    ),

    parameter_list: $ => seq(
      '(',
      optional(seq(
        $.parameter,
        repeat(seq(',', $.parameter)),
        optional(','),
      )),
      ')',
    ),

    parameter: $ => seq(
      optional($.visibility_modifier),
      field('name', $.identifier),
      ':',
      field('type', $._type),
    ),

    visibility_modifier: _ => choice(
      'public',
      'private',
      'constant',
    ),

    function_output: $ => choice(
      seq(optional($.visibility_modifier), $._type),
      seq(
        '(',
        optional($.visibility_modifier),
        $._type,
        repeat(seq(',', optional($.visibility_modifier), $._type)),
        optional(','),
        ')',
      ),
    ),

    constructor_declaration: $ => seq(
      repeat($.annotation),
      'async',
      'constructor',
      '(',
      ')',
      $.block,
    ),

    // ============================================
    // Structs and Records
    // ============================================
    struct_declaration: $ => seq(
      'struct',
      field('name', $.identifier),
      optional($.const_parameters),
      '{',
      optional($.struct_members),
      '}',
    ),

    record_declaration: $ => seq(
      'record',
      field('name', $.identifier),
      '{',
      optional($.struct_members),
      '}',
    ),

    struct_members: $ => seq(
      $.struct_member,
      repeat(seq(',', $.struct_member)),
      optional(','),
    ),

    struct_member: $ => seq(
      optional($.visibility_modifier),
      field('name', $.identifier),
      ':',
      field('type', $._type),
    ),

    // ============================================
    // Mappings and Storage
    // ============================================
    mapping_declaration: $ => seq(
      'mapping',
      field('name', $.identifier),
      ':',
      field('key_type', $._type),
      '=>',
      field('value_type', $._type),
      ';',
    ),

    storage_declaration: $ => seq(
      'storage',
      field('name', $.identifier),
      ':',
      field('type', choice($._type, $.vector_type)),
      ';',
    ),

    // ============================================
    // Global Constants
    // ============================================
    global_const: $ => seq(
      'const',
      field('name', $.identifier),
      ':',
      field('type', $._type),
      '=',
      field('value', $._expression),
      ';',
    ),
  },
});
