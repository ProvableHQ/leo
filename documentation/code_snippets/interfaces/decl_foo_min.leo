// ANCHOR: file
interface Foo {
    record Bar; // programs implementing Foo must declare a record called Bar
}
// ANCHOR_END: file

program decl_foo_min.aleo {
    fn ping() {}

    @noupgrade
    constructor() {}
}
