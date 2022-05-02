/*
namespace: Compile
expectation: Pass
state_file: input/token_withdraw.state
input_file: input/dummy.in
*/

function main() -> bool {
    const expected: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9;

    return input.state.root == [0u8; 32] 
        && input.state_leaf.network_id == 0u8;
}
