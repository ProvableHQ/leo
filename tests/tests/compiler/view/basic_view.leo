program test.aleo {
    mapping balances: address => u64;

    fn main(x: u32) -> u32 {
        return x + 1u32;
    }

    view fn balance_of(addr: address) -> u64 {
        let balance: u64? = balances.get_or_use(addr, 0u64);
        return balance.unwrap_or(0u64);
    }

    @noupgrade
    constructor() {}
}
