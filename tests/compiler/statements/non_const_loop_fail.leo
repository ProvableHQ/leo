/*
namespace: Compile
expectation: Fail
*/

function main() {
    for i in 0..range() {
        let x = 0u8;
    }
}

function range() -> u32 {
    return 10;
}
