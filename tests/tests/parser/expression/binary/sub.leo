/*
namespace: ParseExpression
expectation: Pass
*/

1u8 - 1u8

2u16-3u64

1u32 - 2u32 - 3u32

1u8 * 2u16 - 3u32 * 4i64

!-x.sub(y)
x.sub(y).sub(y).sub(y)
-x.sub(y)
(-x).sub(y)
-(x.sub(y))
(-x.sub(y)).sub(y)
!x.sub(y)
(!x).sub(y)
!(x.sub(y))
(!x.sub(y)).sub(y)
(x.sub(y) << x.sub(y)).sub(x.sub(y) << x.sub(y))