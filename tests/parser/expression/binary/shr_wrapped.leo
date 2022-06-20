/*
namespace: ParseExpression
expectation: Pass
*/

!-x.shr_wrapped(y)
x.shr_wrapped(y).shr_wrapped(y).shr_wrapped(y)
-x.shr_wrapped(y)
(-x).shr_wrapped(y)
-(x.shr_wrapped(y))
(-x.shr_wrapped(y)).shr_wrapped(y)
!x.shr_wrapped(y)
(!x).shr_wrapped(y)
!(x.shr_wrapped(y))
(!x.shr_wrapped(y)).shr_wrapped(y)
(x.shr_wrapped(y) >> x.shr_wrapped(y)).shr_wrapped(x.shr_wrapped(y) >> x.shr_wrapped(y))