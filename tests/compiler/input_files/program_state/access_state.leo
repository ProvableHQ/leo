/*
namespace: Compile
expectation: Pass
state_file: input/token_withdraw.state
input_file: input/dummy.in
*/

function main() -> bool {
    return input.state.root == [0u8; 32];
}
