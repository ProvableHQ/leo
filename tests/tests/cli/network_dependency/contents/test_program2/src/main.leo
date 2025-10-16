import test_program1.aleo;
program test_program2.aleo {
    transition main() -> u32 {
        return test_program1.aleo/main(2u32);
    }

    @noupgrade
    async constructor() {}
}
