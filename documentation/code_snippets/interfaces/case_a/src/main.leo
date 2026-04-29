// ANCHOR: file
interface ARC20 {
    fn transfer_private(token: dyn record, to: address) -> dyn record;
}

program caller.aleo {
    fn main(target: identifier, token: dyn record, to: address) -> dyn record {
        return ARC20@(target)::transfer_private(token, to); // direct pass-through
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
