// ANCHOR: file
interface ARC20 {
    record Token;
    fn transfer_private(token: Token, to: address) -> Token;
}

program caller.aleo {
    fn main(target: identifier, token: dyn record, to: address) -> dyn record {
        return ARC20@(target)::transfer_private(token, to); // implicit cast, returns dyn record
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
