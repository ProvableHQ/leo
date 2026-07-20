program test.aleo {
    fn matches(addr: address) -> bool {
        return std::ctx::caller() == addr;
    }

    @noupgrade
    constructor() {}
}
