/*
namespace: ParseExpression
expectation: Pass
*/

1u8 <= 1u8

2u8 <= 3u8

1u8 + 2u8 <= 3u8 + 4u8

!-x.le(y)
x.le(y).le(y).le(y)
-x.le(y)
(-x).le(y)
-(x.le(y))
(-x.le(y)).le(y)
!x.le(y)
(!x).le(y)
!(x.le(y))
(!x.le(y)).le(y)
(x.le(y) <= x.le(y)).le(x.le(y) <= x.le(y))