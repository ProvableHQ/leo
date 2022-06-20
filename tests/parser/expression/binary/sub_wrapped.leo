/*
namespace: ParseExpression
expectation: Pass
*/

!-x.sub_wrapped(y)
x.sub_wrapped(y).sub_wrapped(y).sub_wrapped(y)
-x.sub_wrapped(y)
(-x).sub_wrapped(y)
-(x.sub_wrapped(y))
(-x.sub_wrapped(y)).sub_wrapped(y)
!x.sub_wrapped(y)
(!x).sub_wrapped(y)
!(x.sub_wrapped(y))
(!x.sub_wrapped(y)).sub_wrapped(y)
(x.sub_wrapped(y) << x.sub_wrapped(y)).sub_wrapped(x.sub_wrapped(y) << x.sub_wrapped(y))