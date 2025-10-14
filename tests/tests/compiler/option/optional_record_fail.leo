program optional_record_test.aleo {

    // === Record ===
    record Foo {
        owner: address,
        value: u64,
    }

    transition handle_optional_record() -> u64 {
        let f: Foo? = Foo {
            owner: self.signer,
            value: 5,
        };
        return 0;
    }

    @noupgrade
    async constructor() {}
}

