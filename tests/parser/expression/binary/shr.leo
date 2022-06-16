/*
namespace: ParseExpression
expectation: Pass
*/

1u8 >> 1u8

2u8>>3u8

1u8 >> 2u8 >> 3u8

1u8 + 2u8 >> 3u8 + 4u8

1u8 >> 2u8 >> 3u8

1u8 + 2u8 >> 3u8 + 4u8 >> 5u8 + 6u8

!-x.shr(y)
x.shr(y).shr(y).shr(y)
-x.shr(y)
(-x).shr(y)
-(x.shr(y))
(-x.shr(y)).shr(y)
!x.shr(y)
(!x).shr(y)
!(x.shr(y))
(!x.shr(y)).shr(y)
(x.shr(y) >> x.shr(y)).shr(x.shr(y) >> x.shr(y))