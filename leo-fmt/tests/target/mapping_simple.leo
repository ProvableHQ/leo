program test.aleo {
    mapping balances: address => u64;

    async transition deposit(amount: u64) -> Future {
        return finalize_deposit(self.caller, amount);
    }

    async function finalize_deposit(caller: address, amount: u64) {
        balances.set(caller, amount);
    }
}
