/*
namespace: ParseExpression
expectation: Pass
*/

1 << 1

2<<3

1 << 2 << 3

1 + 2 << 3 + 4

1 << 2 << 3

1 + 2 << 3 + 4 << 5 + 6

!-x.shl(y)
x.shl(y).shl(y).shl(y)
-x.shl(y)
(-x).shl(y)
-(x.shl(y))
(-x.shl(y)).shl(y)
!x.shl(y)
(!x).shl(y)
!(x.shl(y))
(!x.shl(y)).shl(y)
(x.shl(y) << x.shl(y)).shl(x.shl(y) << x.shl(y))