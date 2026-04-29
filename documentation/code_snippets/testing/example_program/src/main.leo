program example_program.aleo {
    // ANCHOR: record_example
    record Example {
        owner: address,
        x: field,
    }
    // ANCHOR_END: record_example

    // ANCHOR: simple_addition
    fn simple_addition(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
    // ANCHOR_END: simple_addition

    // ANCHOR: mint_record
    fn mint_record(x: field) -> Example {
        return Example {
            owner: self.signer,
            x,
        };
    }
    // ANCHOR_END: mint_record

    @noupgrade
    constructor() {}
}
