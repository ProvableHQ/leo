/*
namespace: ParseExpression
expectation: Pass
*/

1u8 & 1u8

2u8&3u8

1u8 & 2u8 & 3u8

1u8 | 2u8 & 3u8 | 4u8

1u8 & 2u8 & 3u8

1u8 | 2u8 & 3u8 | 4u8 & 5u8 | 6u8

!-x.and(y)
x.and(y).and(y).and(y)
-x.and(y)
(-x).and(y)
-(x.and(y))
(-x.and(y)).and(y)
!x.and(y)
(!x).and(y)
!(x.and(y))
(!x.and(y)).and(y)
(x.and(y) & x.and(y)).and(x.and(y) & x.and(y))