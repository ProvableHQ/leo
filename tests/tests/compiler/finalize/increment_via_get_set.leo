
program test.aleo {
    mapping amounts: address => u128;

    fn increase_self(amount: u128) -> Final {
        let caller = self.caller;
        return final { finalize_increase_self(caller, amount); };
    }
}

final fn finalize_increase_self(addr: address, amount: u128) {
    let current_amount: u128 = Mapping::get_or_use(amounts, addr, 0u128);
    Mapping::set(amounts, addr, current_amount + amount);
}
