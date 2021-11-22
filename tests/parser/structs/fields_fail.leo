/*
namespace: Parse
expectation: Fail
*/

struct X {
    x: u32
    y: u32
}

struct X {
    x: u32,
    y: u32;
    , // recovery witness
}
