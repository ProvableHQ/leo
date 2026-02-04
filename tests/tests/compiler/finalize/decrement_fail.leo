
program test.aleo {
    mapping amounts: address => u128;

    fn decrease_self(amount: u128) -> Final {
        return finalize_dec(self.caller, amount);
    }
}

final fn finalize_dec(addr: address, amount: u128) {
    decrement(amounts, addr, amount);
}
