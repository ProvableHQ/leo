/*
namespace: ParseStatement
expectation: Pass
*/

assert_eq(x, y);

assert_eq(Foo { x: x }, Foo { x: y });

assert_neq(x, y);

assert(false);
