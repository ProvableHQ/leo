/*
namespace: ParseExpression
expectation: Pass
*/

!-x.sqrt()
x.sqrt().sqrt().sqrt()
-x.sqrt()
(-x).sqrt()
-(x.sqrt())
(-x.sqrt()).sqrt()
!x.sqrt()
(!x).sqrt()
!(x.sqrt())
(!x.sqrt()).sqrt()