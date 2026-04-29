// ANCHOR: file
// 4.0
fn helper(a: u32, b: u32) -> u32 {
    return a + b;
}

program test.aleo {
    fn mint(public amount: u32) -> u32 {
        return helper(amount, 1u32);
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
