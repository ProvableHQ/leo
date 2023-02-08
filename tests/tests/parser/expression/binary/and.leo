/*
namespace: ParseExpression
expectation: Pass
*/

true && false

false&&true

true&&false&&true

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
(x.and(y) + x.and(y)).and(x.and(y) + x.and(y))