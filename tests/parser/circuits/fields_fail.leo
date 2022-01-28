/*
namespace: Parse
expectation: Fail
*/

circuit X {
    x: u32
    y: u32
}

circuit X {
    x: u32,
    y: u32;
    , // recovery witness
}
