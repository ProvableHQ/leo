/*
namespace: ParseExpression
expectation: Pass
*/

!-x.square_root()
x.square_root().square_root().square_root()
-x.square_root()
(-x).square_root()
-(x.square_root())
(-x.square_root()).square_root()
!x.square_root()
(!x).square_root()
!(x.square_root())
(!x.square_root()).square_root()