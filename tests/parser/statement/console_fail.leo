/*
namespace: ParseStatement
expectation: Fail
*/

console.error("‪"); // bidi override

console.log(1);

console.test();

console.error("{}", x);

console.error("{}{}", x, y);

console.error("x");

console.assert(true);

console.assert_eq(1u32, 2u32);

console.assert_neq(true, false);

console.log("{}", x);

console.log("{}{}", x, y);

console.log("x");
