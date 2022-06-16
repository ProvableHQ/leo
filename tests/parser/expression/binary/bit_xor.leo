/*
namespace: ParseExpression
expectation: Pass
*/

1 ^ 1

2^3

1 ^ 2 ^ 3

1 & 2 ^ 3 & 4

1 ^ 2 ^ 3

1 & 2 ^ 3 & 4 ^ 5 & 6

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