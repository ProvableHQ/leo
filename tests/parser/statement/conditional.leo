/*
namespace: ParseStatement
expectation: Pass
*/

if x {
    return 0;
}

if (x) {
    return 0;
}

if (x) {} else {}

if x+y {} else if x+z {} else {}

if x+y {
    expr;
    return 0;
}