// ANCHOR: file
// 4.0
program token.aleo {
    mapping balances: address => u64;

    fn mint(public receiver: address, public amount: u64) -> Final {
        return final {
            let current: u64 = balances.get_or_use(receiver, 0u64);
            balances.set(receiver, current + amount);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
