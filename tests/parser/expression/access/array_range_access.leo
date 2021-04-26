/*
namespace: ParseExpression
expectation: Pass
*/

x[..]
x[1..]
x[..1]
x[1..1]
x[0..100]

x[323452345.2345234523453453][323452345.2345234523453453]

x[0u8..1u8]
x[0u8..]
x[..0u8]

x[..]
x[x.y..]
x[..y.x]
x[x.y..y.x]
x[x.y.x..y.x.y]
