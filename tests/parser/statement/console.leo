/*
namespace: ParseStatement
expectation: Pass
*/

console.assert_eq(x, y);

console.assert_eq(Foo { x: x }, Foo { x: y });

console.assert_neq(x, y);

console.assert(false);
