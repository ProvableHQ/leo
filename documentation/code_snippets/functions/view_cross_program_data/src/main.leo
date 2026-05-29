program data.aleo {
    mapping balances: address => u64;

    fn touch(x: u32) -> u32 {
        return x;
    }

    view fn get_balance(account: address) -> u64 {
        return balances.get_or_use(account, 0u64);
    }

    @noupgrade
    constructor() {}
}
