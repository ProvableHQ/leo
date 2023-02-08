/*
namespace: ParseExpression
expectation: Pass
*/

!-x.shl_wrapped(y)
x.shl_wrapped(y).shl_wrapped(y).shl_wrapped(y)
-x.shl_wrapped(y)
(-x).shl_wrapped(y)
-(x.shl_wrapped(y))
(-x.shl_wrapped(y)).shl_wrapped(y)
!x.shl_wrapped(y)
(!x).shl_wrapped(y)
!(x.shl_wrapped(y))
(!x.shl_wrapped(y)).shl_wrapped(y)
(x.shl_wrapped(y) << x.shl_wrapped(y)).shl_wrapped(x.shl_wrapped(y) << x.shl_wrapped(y))