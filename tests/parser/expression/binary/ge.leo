/*
namespace: ParseExpression
expectation: Pass
*/

1u8 >= 1u8

2u8 >= 3u8

1u8 + 2u8 >= 3u8 + 4u8

!-x.gte(y)
x.gte(y).gte(y).gte(y)
-x.gte(y)
(-x).gte(y)
-(x.gte(y))
(-x.gte(y)).gte(y)
!x.gte(y)
(!x).gte(y)
!(x.gte(y))
(!x.gte(y)).gte(y)
(x.gte(y) >= x.gte(y)).gte(x.gte(y) >= x.gte(y))