/*
namespace: ParseExpression
expectation: Pass
*/

!-x.pow_wrapped(y)
x.pow_wrapped(y).pow_wrapped(y).pow_wrapped(y)
-x.pow_wrapped(y)
(-x).pow_wrapped(y)
-(x.pow_wrapped(y))
(-x.pow_wrapped(y)).pow_wrapped(y)
!x.pow_wrapped(y)
(!x).pow_wrapped(y)
!(x.pow_wrapped(y))
(!x.pow_wrapped(y)).pow_wrapped(y)
(x.pow_wrapped(y) ** x.pow_wrapped(y)).pow_wrapped(x.pow_wrapped(y) ** x.pow_wrapped(y))