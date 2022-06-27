/*
namespace: ParseExpression
expectation: Pass
*/

1u8 <= 1u8

2u8 <= 3u8

1u8 + 2u8 <= 3u8 + 4u8

!-x.lte(y)
x.lte(y).lte(y).lte(y)
-x.lte(y)
(-x).lte(y)
-(x.lte(y))
(-x.lte(y)).lte(y)
!x.lte(y)
(!x).lte(y)
!(x.lte(y))
(!x.lte(y)).lte(y)
(x.lte(y) <= x.lte(y)).lte(x.lte(y) <= x.lte(y))