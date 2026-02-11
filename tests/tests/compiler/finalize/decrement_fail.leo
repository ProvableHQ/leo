
program test.aleo {
    mapping amounts: address => u128;

    fn decrease_self(amount: u128) -> Final {
        let caller = self.caller;
        return final { finalize_dec(caller, amount); };
    }
}

final fn finalize_dec(addr: address, amount: u128) {
    decrement(amounts, addr, amount);
}
