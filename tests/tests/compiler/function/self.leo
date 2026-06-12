program test.aleo {
    fn matches(addr: address) -> bool {
        return self.caller == addr;
    }

    @noupgrade
    constructor() {}
}
