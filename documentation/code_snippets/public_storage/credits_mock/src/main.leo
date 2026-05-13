program credits.aleo {
    mapping account: address => u64;

    fn touch(addr: address, value: u64) -> Final {
        return final {
            Mapping::set(account, addr, value);
        };
    }

    @noupgrade
    constructor() {}
}
