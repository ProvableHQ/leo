/*
namespace: Compile
expectation: Fail
input_file: input/dummy.in
*/

function main() -> bool {
    if false {
        const a = 1u16;
        const b = a + 1u16;
        return b == 2u16;
    } else if false {
        return true;
    }
}
