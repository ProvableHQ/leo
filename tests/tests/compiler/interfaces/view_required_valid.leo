// Positive case: interface declares a `view fn`, the program implements it
// with the matching `view fn` signature. Should compile cleanly.
export interface IToken {
    view fn balance_of(account: address) -> u128;
    fn transfer(public to: address, public amount: u128) -> Final;
}

program scratch.aleo: IToken {
    mapping balances: address => u128;

    @noupgrade
    constructor() {}

    view fn balance_of(account: address) -> u128 {
        return balances.get_or_use(account, 0u128);
    }

    fn transfer(public to: address, public amount: u128) -> Final {
        return final { };
    }
}
