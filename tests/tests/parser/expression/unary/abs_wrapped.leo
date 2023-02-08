/*
namespace: ParseExpression
expectation: Pass
*/

x.abs_wrapped()
x.abs_wrapped().abs_wrapped().abs_wrapped()
-x.abs_wrapped()
(-x).abs_wrapped()
-(x.abs_wrapped())
(-x.abs_wrapped()).abs_wrapped()
!x.abs_wrapped()
(!x).abs_wrapped()
!(x.abs_wrapped())
(!x.abs_wrapped()).abs_wrapped()