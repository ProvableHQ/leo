// ANCHOR: file
interface Foo {
    record Bar {
        owner: address,
        baz: u64,
        ..
    }
}

// ANCHOR_END: file

program decl_foo_fields.aleo {
    fn ping() {}

    @noupgrade
    constructor() {}
}
