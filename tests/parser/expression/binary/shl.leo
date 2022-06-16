/*
namespace: ParseExpression
expectation: Pass
*/

1u8 << 1u8

2u8<<3u8

1u8 << 2u8 << 3u8

1u8 + 2u8 << 3u8 + 4u8

1u8 << 2u8 << 3u8

1u8 + 2u8 << 3u8 + 4u8 << 5u8 + 6u8

!-x.shl(y)
x.shl(y).shl(y).shl(y)
-x.shl(y)
(-x).shl(y)
-(x.shl(y))
(-x.shl(y)).shl(y)
!x.shl(y)
(!x).shl(y)
!(x.shl(y))
(!x.shl(y)).shl(y)
(x.shl(y) << x.shl(y)).shl(x.shl(y) << x.shl(y))