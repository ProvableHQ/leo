program test_program1.aleo {
    transition main(b: u32) -> u32 {
        return b;
    }

    @noupgrade
    async constructor() {}
}
