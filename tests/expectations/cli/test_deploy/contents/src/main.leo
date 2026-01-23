// The 'some_sample_leo_program' program.
program some_sample_leo_program.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

    @noupgrade
    constructor() {}
}
