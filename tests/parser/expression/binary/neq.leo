/*
namespace: ParseExpression
expectation: Pass
*/

1u8 != 1u8

2u8!=3u8

1u8 < 2u8 != 3u8 < 4u8

1u8 < 2u8 != 3u8 < 4u8 || 3u8 < 4u8 != 5u8 < 6u8

!-x.neq(y)
x.neq(y).neq(y).neq(y)
-x.neq(y)
(-x).neq(y)
-(x.neq(y))
(-x.neq(y)).neq(y)
!x.neq(y)
(!x).neq(y)
!(x.neq(y))
(!x.neq(y)).neq(y)
(x.neq(y) != x.neq(y)).neq(x.neq(y) != x.neq(y))