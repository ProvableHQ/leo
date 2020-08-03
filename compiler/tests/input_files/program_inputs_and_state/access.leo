function main(
    registers,
    state,
    record,
    state_leaf,
    data: u8[32]
) {
    assert_eq!(registers.value_balance, 0u64);

    assert_eq!(state.leaf_index, 0u32);

    assert_eq!(record.value, 5u64);

    assert_eq!(state_leaf.network_id, 0u8);

    assert_eq!(data, [0u8; 32]);
}