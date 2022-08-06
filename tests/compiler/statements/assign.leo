/*
namespace: Compile
expectation: Pass
input_file: inputs/flag.in
*/

@program
function main(flag: u8, value: u8) -> u8 {
    if (flag == 0u8) {
        value += 1u8;
        return value;
    } else {
        value += 2u8;
    }
    return value;
}