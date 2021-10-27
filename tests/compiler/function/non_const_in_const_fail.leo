/*
namespace: Compile
expectation: Fail
*/

function main() {
    for i in 0..iter() {
        let x = 0u8;
    }
}

const function iter() -> u32 {
    return read_input() ? 10 : 5;
}

function read_input() -> bool {
    return input.record.is_dummy;
}
