/*
namespace: ParseExpression
expectation: Pass
*/

x[..]
x[1u8..]
x[..1u8]
x[1u8..1u8]
x[0u8..100u8]

x[323452345u8.2345234523453453][323452345u8.2345234523453453]

x[0u8..1u8]
x[0u8..]
x[..0u8]

x[..]
x[x.y..]
x[..y.x]
x[x.y..y.x]
x[x.y.x..y.x.y]
