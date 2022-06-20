/*
namespace: ParseExpression
expectation: Pass
*/

1u8 < 1u8

2u8<3u8

1u8 + 2u8 < 3u8 + 4u8

!-x.lt(y)
x.lt(y).lt(y).lt(y)
-x.lt(y)
(-x).lt(y)
-(x.lt(y))
(-x.lt(y)).lt(y)
!x.lt(y)
(!x).lt(y)
!(x.lt(y))
(!x.lt(y)).lt(y)
(x.lt(y) < x.lt(y)).lt(x.lt(y) < x.lt(y))