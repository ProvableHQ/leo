// ANCHOR: file
// The 'noupgrade_example' program.
program noupgrade_example.aleo {
    // This constructor is for the "noupgrade" mode.
    // It is immutable and prevents any future upgrades.
    @noupgrade
    constructor() {
        // The Leo compiler automatically generates the constructor logic.
    }

    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
}
// ANCHOR_END: file
