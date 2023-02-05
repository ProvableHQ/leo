/*
namespace: ParseExpression
expectation: Pass
*/

1u8 + 1u8
2u8+3u8
1u8 + 2u8 + 3u8
1u8 * 2u8 + 3u8 * 4u8
1u8 + 2u8 - 3u8
1u8 * 2u8 + 3u8 * 4u8 - 5u8 * 6u8

!-x.add(y)
x.add(y).add(y).add(y)
-x.add(y)
(-x).add(y)
-(x.add(y))
(-x.add(y)).add(y)
!x.add(y)
(!x).add(y)
!(x.add(y))
(!x.add(y)).add(y)
(x.add(y) + x.add(y)).add(x.add(y) + x.add(y))