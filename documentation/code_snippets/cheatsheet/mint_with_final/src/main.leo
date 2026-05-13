// ANCHOR: mint_with_final
program example.aleo {
    mapping balances: address => u64;

    fn mint(receiver: address, amount: u64) -> Final {
        return final {
            let current_balance: u64 = balances.get_or_use(receiver, 0u64);
            balances.set(receiver, current_balance + amount);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: mint_with_final
