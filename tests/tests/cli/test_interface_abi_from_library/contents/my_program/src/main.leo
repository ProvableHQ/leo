program my_program.aleo : my_lib::Greeter {
    fn greet(who: address) -> u64 {
        return 1u64;
    }

    @noupgrade
    constructor() {}
}
