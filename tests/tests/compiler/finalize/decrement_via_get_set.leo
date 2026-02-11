
program test.aleo {
    mapping amounts: address => u128;

    fn decrease_self(amount: u128) -> Final {
        let caller = self.caller;
        return final { finalize_dec(caller, amount); };
    }
}

final fn finalize_dec(addr: address, amount: u128) {
    let current_amount: u128 = Mapping::get_or_use(amounts, addr, 0u128);
    Mapping::set(amounts, addr, current_amount - amount);
}
