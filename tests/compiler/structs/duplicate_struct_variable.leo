/*
namespace: Compile
expectation: Fail
*/

struct Bar {
    x: u32,
    x: u32,
}

function main() -> bool {
    return true;
}
