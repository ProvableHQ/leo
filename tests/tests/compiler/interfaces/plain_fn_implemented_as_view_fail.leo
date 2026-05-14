// FAIL: the inverse of `view_required_wrong_variant_fail` — interface declares
// a plain `fn`, the program tries to satisfy it with a `view fn`. Variants
// must match in both directions.
interface IToken {
    fn read(account: address) -> u128;
}

program scratch.aleo: IToken {
    mapping balances: address => u128;

    @noupgrade
    constructor() {}

    view fn read(account: address) -> u128 {
        return balances.get_or_use(account, 0u128);
    }
}
