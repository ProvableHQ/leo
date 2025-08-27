program optional_record_test.aleo {

    // === Record ===
    record Foo {
        owner: address,
        value: u64,
    }

    // === Transition that takes Foo? as input ===
    transition handle_optional_record(opt: Foo?) -> u64 {
        return 0;
    }

    @noupgrade
    async constructor() {}
}

