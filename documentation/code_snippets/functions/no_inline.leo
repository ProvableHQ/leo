// ANCHOR: snippet
@no_inline
fn expensive_helper(a: u32, b: u32) -> u32 {
    // ...
    return a + b;
}
// ANCHOR_END: snippet

program no_inline_demo.aleo {
    fn use_helper(a: u32, b: u32) -> u32 {
        return expensive_helper(a, b);
    }

    @noupgrade
    constructor() {}
}
