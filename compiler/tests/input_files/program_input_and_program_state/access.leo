function main(input, data: u8[32]) {
    console.assert(input.registers.value_balance == 0u64);

    console.assert(input.state.leaf_index == 0u32);

    console.assert(input.record.value == 5u64);

    console.assert(input.state_leaf.network_id == 0u8);

    console.assert(data == [0u8; 32]);
}