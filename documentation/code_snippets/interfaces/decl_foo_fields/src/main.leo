// ANCHOR: file
interface Foo {
    record Bar {
        owner: address, // all records must have an owner field
        baz: u64,       // Bar must also have a baz field of type u64
        ..              // implementors may add more fields
    }
}
// ANCHOR_END: file

program decl_foo_fields.aleo {
    fn ping() {}

    @noupgrade
    constructor() {}
}
