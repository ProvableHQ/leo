// ANCHOR: snippet
fn foo(
    a: field,
    b: field,
) -> field {
    return a + b;
}
// ANCHOR_END: snippet

program helper_basic.aleo {
    fn use_helper(a: field, b: field) -> field {
        return foo(a, b);
    }

    @noupgrade
    constructor() {}
}
