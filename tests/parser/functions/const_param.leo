/*
ns: Parse
expectation: Pass
*/

function x(x: u32, const y: i32) {
    return ();
}

function x(const x: u32, y: i32) {
    return ();
}