/*
namespace: ParseExpression
expectation: Pass
*/

1u8 % 1u8

2u8%3u8

1u8 % 2u8 % 3u8

1u8 ** 2u8 % 3u8 ** 4u8

!-x.rem(y)
x.rem(y).rem(y).rem(y)
-x.rem(y)
(-x).rem(y)
-(x.rem(y))
(-x.rem(y)).rem(y)
!x.rem(y)
(!x).rem(y)
!(x.rem(y))
(!x.rem(y)).rem(y)
(x.rem(y) % x.rem(y)).rem(x.rem(y) % x.rem(y))