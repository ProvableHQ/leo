/*
namespace: ParseExpression
expectation: Pass
*/

foo as u8

1u128 as i8

baz { foo: u8 } as scalar

flag ? 1u8 : 0u8 as scalar

(flag ? 1u8 : 0u8) as scalar

34i8 as field

0925348043850field as i8
