program test_failure_exit_code.aleo {
    transition main() -> u8 {
        return 42u8;
    }

    @noupgrade
    async constructor() {}
}
