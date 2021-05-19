/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

function main() -> (bool, bool) {
    return (input.registers.r0, input.registers.r1);
}
