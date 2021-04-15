function main() {
    console.assert(input.state.root == [0u8; 32]);

    const expected: address = aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8;
    //console.assert(input.record.owner, expected);

    console.assert(input.state_leaf.network_id == 0u8);
}