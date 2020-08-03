function main(input, data: u8[32]) {
    assert_eq!(input.registers.value_balance, 0u64);

    assert_eq!(input.state.leaf_index, 0u32);

    assert_eq!(input.record.value, 5u64);

    assert_eq!(input.state_leaf.network_id, 0u8);

    assert_eq!(data, [0u8; 32]);
}