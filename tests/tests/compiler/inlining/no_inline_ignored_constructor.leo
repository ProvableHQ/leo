@no_inline
fn helper(x: u8) -> u8 {
    return x + 1u8;
}

program test.aleo {
    mapping foo: u8 => u8;

    fn noop() {}

    @custom
    constructor() {
        let val: u8 = helper(0u8);
        foo.set(0u8, val);
    }
}
