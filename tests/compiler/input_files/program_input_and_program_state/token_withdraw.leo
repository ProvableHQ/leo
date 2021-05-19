/*
namespace: Compile
expectation: Pass
state_file: input/token_withdraw.state
input_file: input/token_withdraw.in
*/

function main(data: [u8; 32]) -> bool {
    return input.registers.value_balance == 0u64
        && input.state.leaf_index == 0u32
        && input.record.value == 5u64
        && input.state_leaf.network_id == 0u8
        && data == [0u8; 32];
}
