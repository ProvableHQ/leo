program entry_output.aleo {
    // ANCHOR: snippet
    fn foo(public a: field) -> field {
        // Returns the addition of the public input a and the value `1field`.
        return a + 1field;
    }
    // ANCHOR_END: snippet

    @noupgrade
    constructor() {}
}
