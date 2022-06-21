/*
namespace: ParseExpression
expectation: Pass
*/

!-x.nand(y)
x.nand(y).nand(y).nand(y)
-x.nand(y)
(-x).nand(y)
-(x.nand(y))
(-x.nand(y)).nand(y)
!x.nand(y)
(!x).nand(y)
!(x.nand(y))
(!x.nand(y)).nand(y)
(x.nand(y) & x.nand(y)).nand(x.nand(y) & x.nand(y))