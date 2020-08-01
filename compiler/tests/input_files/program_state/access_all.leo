function main(input) {
    assert_eq!(input.state.root, [0u8; 32]);

    let expected: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
    assert_eq!(input.record.owner, expected);

    assert_eq!(input.state_leaf.network_id, 0u8);
}