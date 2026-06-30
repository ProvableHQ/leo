export final fn bar() {
    let x: u32 = std::ctx::block_timestamp();
}

program test.aleo {
    fn foo() -> Final {
        return final { bar(); };
    }

    @noupgrade
    constructor() {}
}
