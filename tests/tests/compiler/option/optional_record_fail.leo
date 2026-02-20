
program optional_record_test.aleo {
    // === Record ===
    record Foo {
        owner: address,
        val: u64,
    }

    fn handle_optional_record() -> u64 {
        let f: Foo? = Foo {
            owner: self.signer,
            val: 5,
        };
        return 0;
    }

    @noupgrade
    constructor() {}
}

