// Reproduces issue #29421: a top-level `final fn` that is never called from
// any `final {}` block should warn.

program test.aleo {
    mapping balances: address => u128;

    fn deposit(public amount: u64) -> Final {
        let depositor = self.signer;
        return final {
            add_supply(depositor, amount as u128);
        };
    }

    @noupgrade
    constructor() {}
}

// Called from `deposit`. Should not warn.
final fn add_supply(addr: address, amount: u128) {
    let prev = balances.get_or_use(addr, 0u128);
    balances.set(addr, prev + amount);
}

// Orphan: never called from any final block. Should warn.
final fn set_supply(addr: address, new_supply: u128) {
    balances.set(addr, new_supply);
}
