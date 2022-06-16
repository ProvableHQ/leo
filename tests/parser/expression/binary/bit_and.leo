/*
namespace: ParseExpression
expectation: Pass
*/

1 & 1

2&3

1 & 2 & 3

1 == 2 & 3 == 4

1 & 2 & 3

1 == 2 & 3 == 4 & 5 == 6

!-x.and(y)
x.and(y).and(y).and(y)
-x.and(y)
(-x).and(y)
-(x.and(y))
(-x.and(y)).and(y)
!x.and(y)
(!x).and(y)
!(x.and(y))
(!x.and(y)).and(y)
(x.and(y) & x.and(y)).and(x.and(y) & x.and(y))