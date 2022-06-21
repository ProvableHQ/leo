/*
namespace: ParseExpression
expectation: Pass
*/

1u8 ** 1u8

2u8**3u8

1u8 ** 2u8 ** 3u8

!-x.pow(y)
x.pow(y).pow(y).pow(y)
-x.pow(y)
(-x).pow(y)
-(x.pow(y))
(-x.pow(y)).pow(y)
!x.pow(y)
(!x).pow(y)
!(x.pow(y))
(!x.pow(y)).pow(y)
(x.pow(y) ** x.pow(y)).pow(x.pow(y) ** x.pow(y))