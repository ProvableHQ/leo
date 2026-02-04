program test_shortcuts.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

    @noupgrade
    constructor() {}
}
