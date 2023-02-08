/*
namespace: ParseExpression
expectation: Pass
*/

!-x.mod(y)
x.mod(y).mod(y).mod(y)
-x.mod(y)
(-x).mod(y)
-(x.mod(y))
(-x.mod(y)).mod(y)
!x.mod(y)
(!x).mod(y)
!(x.mod(y))
(!x.mod(y)).mod(y)
(x.mod(y) / x.mod(y)).mod(x.mod(y) / x.mod(y))