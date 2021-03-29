/*
ns: ParseStatement
expectation: Pass
*/

console.assert(x);


console.error("{}", x);

console.error("{}{}", x, y);

console.error("x");


console.debug("{}", x);

console.debug("{}{}", x, y);

console.debug("x");


console.log("{}", x);

console.log("{}{}", x, y);

console.log("x");