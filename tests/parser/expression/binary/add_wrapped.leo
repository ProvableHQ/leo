/*
namespace: ParseExpression
expectation: Pass
*/

!-x.add_wrapped(y)
x.add_wrapped(y).add_wrapped(y).add_wrapped(y)
-x.add_wrapped(y)
(-x).add_wrapped(y)
-(x.add_wrapped(y))
(-x.add_wrapped(y)).add_wrapped(y)
!x.add_wrapped(y)
(!x).add_wrapped(y)
!(x.add_wrapped(y))
(!x.add_wrapped(y)).add_wrapped(y)
(x.add_wrapped(y) + x.add_wrapped(y)).add_wrapped(x.add_wrapped(y) + x.add_wrapped(y))