/*
namespace: ParseExpression
expectation: Pass
*/

x as u8
y as id
z as u32
x as i128
x as u8 as u128
x as field
x as group
x ** y as u32 ** z
// ~x as u32
!x as u32
-x as u32