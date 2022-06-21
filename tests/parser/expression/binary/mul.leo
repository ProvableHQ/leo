/*
namespace: ParseExpression
expectation: Pass
*/

1u8 * 1u8

2u8*3u8

1u8 * 2u8 * 3u8

1u8 ** 2u8 * 3u8 ** 4u8

1u8 ** 2u8 * 3u8 ** 4u8 / 5u8 ** 6u8

!-x.mul(y)
x.mul(y).mul(y).mul(y)
-x.mul(y)
(-x).mul(y)
-(x.mul(y))
(-x.mul(y)).mul(y)
!x.mul(y)
(!x).mul(y)
!(x.mul(y))
(!x.mul(y)).mul(y)
(x.mul(y) * x.mul(y)).mul(x.mul(y) * x.mul(y))