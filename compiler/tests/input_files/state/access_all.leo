function main(state, record, state_leaf) {
    assert_eq!(state.root, [0u8; 32]);

    let expected: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
    assert_eq!(record.owner, expected);

    assert_eq!(state_leaf.network_id, 0u8);
}