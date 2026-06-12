// The 'consumer' program.
program consumer.aleo {
    @noupgrade
    constructor() {}

    fn main(a: u32, b: u32) -> u32 {
        return a + b;
    }
}
