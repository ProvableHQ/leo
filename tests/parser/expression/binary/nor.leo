/*
namespace: ParseExpression
expectation: Pass
*/

!-x.nor(y)
x.nor(y).nor(y).nor(y)
-x.nor(y)
(-x).nor(y)
-(x.nor(y))
(-x.nor(y)).nor(y)
!x.nor(y)
(!x).nor(y)
!(x.nor(y))
(!x.nor(y)).nor(y)
(x.nor(y) != x.nor(y)).nor(x.nor(y) != x.nor(y))