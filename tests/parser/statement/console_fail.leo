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


console.log("{}", x);

console.log("{}{}", x, y);

console.log("x");
