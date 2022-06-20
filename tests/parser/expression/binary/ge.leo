/*
namespace: ParseExpression
expectation: Pass
*/

1u8 >= 1u8

2u8 >= 3u8

1u8 + 2u8 >= 3u8 + 4u8

!-x.ge(y)
x.ge(y).ge(y).ge(y)
-x.ge(y)
(-x).ge(y)
-(x.ge(y))
(-x.ge(y)).ge(y)
!x.ge(y)
(!x).ge(y)
!(x.ge(y))
(!x.ge(y)).ge(y)
(x.ge(y) >= x.ge(y)).ge(x.ge(y) >= x.ge(y))