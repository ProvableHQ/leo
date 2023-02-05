/*
namespace: ParseExpression
expectation: Pass
*/

!-x.rem_wrapped(y)
x.rem_wrapped(y).rem_wrapped(y).rem_wrapped(y)
-x.rem_wrapped(y)
(-x).rem_wrapped(y)
-(x.rem_wrapped(y))
(-x.rem_wrapped(y)).rem_wrapped(y)
!x.rem_wrapped(y)
(!x).rem_wrapped(y)
!(x.rem_wrapped(y))
(!x.rem_wrapped(y)).rem_wrapped(y)
(x.rem_wrapped(y) / x.rem_wrapped(y)).rem_wrapped(x.rem_wrapped(y) / x.rem_wrapped(y))