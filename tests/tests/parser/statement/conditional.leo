/*
namespace: ParseStatement
expectation: Pass
*/

if x {
    return 0u8;
}

if (x) {
    return 0u8;
}

if (x) {} else {}

if x+y {} else if x+z {} else {}

if x+y {
    return 0u8;
}
