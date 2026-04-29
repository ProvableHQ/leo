// ANCHOR: file
interface ARC20 {
    fn transfer_private(token: dyn record, to: address) -> dyn record;
}

program my_token.aleo {
    record Token { owner: address, amount: u64 }

    fn do_transfer(target: identifier, token: Token, to: address) -> dyn record {
        return ARC20@(target)::transfer_private(token as dyn record, to); // explicit cast
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
