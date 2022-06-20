/*
namespace: ParseExpression
expectation: Pass
*/

1u8 / 1u8

2u8/3u8

1u8 / 2u8 / 3u8

1u8 ** 2u8 / 3u8 ** 4u8

!-x.div(y)
x.div(y).div(y).div(y)
-x.div(y)
(-x).div(y)
-(x.div(y))
(-x.div(y)).div(y)
!x.div(y)
(!x).div(y)
!(x.div(y))
(!x.div(y)).div(y)
(x.div(y) / x.div(y)).div(x.div(y) / x.div(y))