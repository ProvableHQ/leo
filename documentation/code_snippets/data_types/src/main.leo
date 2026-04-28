// Compilable scaffold so each anchored snippet from `documentation/language/03_data_types.md`
// builds end to end. Anchors carve out the parts shown in the doc.
program data_types_demo.aleo {
    // ANCHOR: token_record
    record Token {
        // The token owner.
        owner: address,
        // The token amount.
        amount: u64,
    }
    // ANCHOR_END: token_record

    @noupgrade
    constructor() {}

    fn mint(amount: u64) -> Token {
        return Token {
            owner: self.signer,
            amount,
        };
    }
}
