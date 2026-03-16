// The 'test_failure_exit_code' program.
program test_failure_exit_code.aleo {
    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }

    @noupgrade
    constructor() {}
}
