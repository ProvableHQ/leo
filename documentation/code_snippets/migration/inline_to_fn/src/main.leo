// ANCHOR: file
// 4.0
fn helper() -> u32 {
    return 42u32;
}

program test.aleo {
    fn mint() -> u32 {
        return helper();
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
