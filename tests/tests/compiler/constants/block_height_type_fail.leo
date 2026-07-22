export final fn bar() {
    let x: u64 = std::ctx::block_height();
}

program test.aleo {
    fn foo() -> Final {
        return final { bar(); };
    }

    @noupgrade
    constructor() {}
}
