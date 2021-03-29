/*
ns: ParseStatement
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

if (x) {}

if x+y {}

if x+y {
    expr;
    return ();
}