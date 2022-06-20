/*
namespace: ParseExpression
expectation: Pass
*/

!-x.mul_wrapped(y)
x.mul_wrapped(y).mul_wrapped(y).mul_wrapped(y)
-x.mul_wrapped(y)
(-x).mul_wrapped(y)
-(x.mul_wrapped(y))
(-x.mul_wrapped(y)).mul_wrapped(y)
!x.mul_wrapped(y)
(!x).mul_wrapped(y)
!(x.mul_wrapped(y))
(!x.mul_wrapped(y)).mul_wrapped(y)
(x.mul_wrapped(y) * x.mul_wrapped(y)).mul_wrapped(x.mul_wrapped(y) * x.mul_wrapped(y))