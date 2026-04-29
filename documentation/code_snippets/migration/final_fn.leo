// ANCHOR: file
// 4.0
final fn update_balance(receiver: address, amount: u64) {
    let current: u64 = balances.get_or_use(receiver, 0u64);
    balances.set(receiver, current + amount);
}

program token.aleo {
    mapping balances: address => u64;

    fn mint(public receiver: address, public amount: u64) -> Final {
        return final {
            update_balance(receiver, amount);
        };
    }

    fn airdrop(public r1: address, public r2: address, public amount: u64) -> Final {
        return final {
            update_balance(r1, amount);
            update_balance(r2, amount);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
