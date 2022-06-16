/*
namespace: ParseExpression
expectation: Pass
*/

1 >> 1

2>>3

1 >> 2 >> 3

1 + 2 >> 3 + 4

1 >> 2 >> 3

1 + 2 >> 3 + 4 >> 5 + 6

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