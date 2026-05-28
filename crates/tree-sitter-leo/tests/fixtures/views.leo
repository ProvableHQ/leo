interface IReader {
    view fn balance_of(account: address) -> u64;
}

program reader.aleo: IReader {
    mapping balances: address => u64;

    view fn balance_of(account: address) -> u64 {
        return balances.get_or_use(account, 0u64);
    }

    fn cache_total(account: address) -> Final {
        return final {
            let bal: u64 = balance_of(account);
            assert(bal >= 0u64);
        };
    }

    @noupgrade
    constructor() {}
}
