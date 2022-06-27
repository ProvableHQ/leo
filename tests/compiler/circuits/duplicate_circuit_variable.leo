/*
namespace: Compile
expectation: Fail
*/

circuit Bar {
    x: u32,
    x: u32,
}

function main() -> bool {
    return true;
}
