/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    if true {} else {}
    let c: u8 = 1u8;
    if true {
        if true {
            c = c;
        }
    }
    return c == 1u8;
}
