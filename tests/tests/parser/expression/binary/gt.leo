/*
namespace: ParseExpression
expectation: Pass
*/

1u8 > 1u8

2u8>3u8

1u8 + 2u8 > 3u8 + 4u8

!-x.gt(y)
x.gt(y).gt(y).gt(y)
-x.gt(y)
(-x).gt(y)
-(x.gt(y))
(-x.gt(y)).gt(y)
!x.gt(y)
(!x).gt(y)
!(x.gt(y))
(!x.gt(y)).gt(y)
(x.gt(y) > x.gt(y)).gt(x.gt(y) > x.gt(y))