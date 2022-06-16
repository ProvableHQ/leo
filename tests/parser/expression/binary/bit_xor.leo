/*
namespace: ParseExpression
expectation: Pass
*/

1u8 ^ 1u8

2u8^3u8

1u8 ^ 2u8 ^ 3u8

1u8 & 2u8 ^ 3u8 & 4u8

1u8 ^ 2u8 ^ 3u8

1u8 & 2u8 ^ 3u8 & 4u8 ^ 5u8 & 6u8

!-x.xor(y)
x.xor(y).xor(y).xor(y)
-x.xor(y)
(-x).xor(y)
-(x.xor(y))
(-x.xor(y)).xor(y)
!x.xor(y)
(!x).xor(y)
!(x.xor(y))
(!x.xor(y)).xor(y)
(x.xor(y) ^ x.xor(y)).xor(x.xor(y) ^ x.xor(y))