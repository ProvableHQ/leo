/*
namespace: ParseExpression
expectation: Pass
*/

!-x.square()
x.square().square().square()
-x.square()
(-x).square()
-(x.square())
(-x.square()).square()
!x.square()
(!x).square()
!(x.square())
(!x.square()).square()