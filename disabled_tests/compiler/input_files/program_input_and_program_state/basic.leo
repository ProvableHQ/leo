/*
namespace: Compile
expectation: Pass
state_file: input/basic.state
input_file: input/basic.in
*/

function main(a: bool) -> bool {
    return a == input.registers.b;
}
