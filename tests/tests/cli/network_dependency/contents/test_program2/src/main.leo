import test_program1.aleo;
program test_program2.aleo {
    fn main() -> u32 {
        return test_program1.aleo/main(2u32);
    }

    @noupgrade
    constructor() {}
}
