// FAIL: an interface declares `view fn balance_of`, the program implements
// `balance_of` but as a regular `fn` instead of a `view fn`. The variant
// check inside `function_matches_prototype` must reject this.
export interface IToken {
    view fn balance_of(account: address) -> u128;
}

program scratch.aleo: IToken {
    mapping balances: address => u128;

    @noupgrade
    constructor() {}

    fn balance_of(account: address) -> u128 {
        return 0u128;
    }
}
