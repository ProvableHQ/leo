/*
namespace: Compile
expectation: Pass
input_file: inputs/dummy.in
*/

function main () -> bool {
    if 1u8 <= (true? 1u8 : 2u8) {
        return true;
    } else  {
        return false;
    }
}