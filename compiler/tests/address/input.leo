function main(owner: address) {
    let sender = address(aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8sta57j8);

    console.assert(owner == sender);
}