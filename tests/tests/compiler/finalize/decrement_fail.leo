
program test.aleo {
    mapping amounts: address => u128;

    async fn decrease_self(amount: u128) -> Future {
        return finalize_dec(self.caller, amount);
    }
}

async fn finalize_dec(addr: address, amount: u128) {
    decrement(amounts, addr, amount);
}
