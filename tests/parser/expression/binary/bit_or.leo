/*
namespace: ParseExpression
expectation: Pass
*/

1 | 1

2|3

1 | 2 | 3

1 ^ 2 | 3 ^ 4

1 | 2 | 3

1 ^ 2 | 3 ^ 4 | 5 ^ 6

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