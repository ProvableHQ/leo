/*
namespace: ParseExpression
expectation: Pass
*/

1u8 | 1u8

2u8|3u8

1u8 | 2u8 | 3u8

1u8 ^ 2u8 | 3u8 ^ 4u8

1u8 | 2u8 | 3u8

1u8 ^ 2u8 | 3u8 ^ 4u8 | 5u8 ^ 6u8

!-x.or(y)
x.or(y).or(y).or(y)
-x.or(y)
(-x).or(y)
-(x.or(y))
(-x.or(y)).or(y)
!x.or(y)
(!x).or(y)
!(x.or(y))
(!x.or(y)).or(y)
(x.or(y) | x.or(y)).or(x.or(y) | x.or(y))