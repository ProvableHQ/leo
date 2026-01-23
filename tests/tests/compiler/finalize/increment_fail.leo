
program test.aleo {
    mapping amounts: address => u128;

    fn increase_self(amount: u128) -> Final {
        return finalize_increase_self(self.caller, amount);
    }
}

onchain fn finalize_increase_self(addr: address, amount: u128) {
    increment(amounts, addr, amount);
}
