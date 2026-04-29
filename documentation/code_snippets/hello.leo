// The 'hello' program.
program hello.aleo {
    // ANCHOR: constructor
    @noupgrade
    constructor() {}
    // ANCHOR_END: constructor

    // ANCHOR: signature
    fn main(public a: u32, b: u32) -> u32 {
        // ANCHOR_END: signature
        // ANCHOR: addition
        let c: u32 = a + b;
        // ANCHOR_END: addition
        // ANCHOR: ret
        return c;
        // ANCHOR_END: ret
    }
}
