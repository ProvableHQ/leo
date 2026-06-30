// FAIL: interface declares `view fn balance_of(addr: address) -> u128`, the
// program implements `view fn balance_of(addr: address) -> u64`. The output
// type mismatch must be caught by the same signature check that already
// covers regular `fn`s.
export interface IToken {
    view fn balance_of(account: address) -> u128;
}

program scratch.aleo: IToken {
    mapping balances: address => u128;

    @noupgrade
    constructor() {}

    view fn balance_of(account: address) -> u64 {
        return 0u64;
    }
}
