program test.aleo {
    mapping amounts: address => u128;

    fn increase_self(amount: u128) -> Final {
        let caller = std::ctx::caller();
        return final { finalize_increase_self(caller, amount); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_increase_self(addr: address, amount: u128) {
    increment(amounts, addr, amount);
}
