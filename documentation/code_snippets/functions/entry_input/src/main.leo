program entry_input.aleo {
    // ANCHOR: snippet
    // The entry function `foo` takes a single input `a` with type `field` and visibility `public`.
    fn foo(public a: field) { }
    // ANCHOR_END: snippet

    @noupgrade
    constructor() {}
}
