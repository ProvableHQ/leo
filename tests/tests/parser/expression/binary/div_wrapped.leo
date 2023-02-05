/*
namespace: ParseExpression
expectation: Pass
*/

!-x.div(y)
x.div(y).div(y).div(y)
-x.div(y)
(-x).div(y)
-(x.div(y))
(-x.div(y)).div(y)
!x.div(y)
(!x).div(y)
!(x.div(y))
(!x.div(y)).div(y)
(x.div(y) / x.div(y)).div(x.div(y) / x.div(y))