// FAIL: an interface declares a `view fn` prototype, the program implements
// the regular fn but omits the view. The missing-function check must apply to
// view fns just like to regular fns.
export interface IToken {
    view fn balance_of(account: address) -> u128;
    fn transfer(public to: address, public amount: u128) -> Final;
}

program scratch.aleo: IToken {
    @noupgrade
    constructor() {}

    fn transfer(public to: address, public amount: u128) -> Final {
        return final { };
    }
}
