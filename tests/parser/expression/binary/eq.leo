/*
namespace: ParseExpression
expectation: Pass
*/

1u8 == 1u8

2u8==3u8

1u8 < 2u8 == 3u8 < 4u8

1u8 < 2u8 == 3u8 < 4u8 && 3u8 < 4u8 == 5u8 < 6u8

!-x.eq(y)
x.eq(y).eq(y).eq(y)
-x.eq(y)
(-x).eq(y)
-(x.eq(y))
(-x.eq(y)).eq(y)
!x.eq(y)
(!x).eq(y)
!(x.eq(y))
(!x.eq(y)).eq(y)
(x.eq(y) == x.eq(y)).eq(x.eq(y) == x.eq(y))