/*
namespace: ParseStatement
expectation: Pass
*/

if x {
    return ();
}

if Self {
    return ();
}

if (x) {
    return ();
}

if (x) {} else {}

if x+y {} else if x+z {} else {}

if x+y {
    expr;
    return ();
}