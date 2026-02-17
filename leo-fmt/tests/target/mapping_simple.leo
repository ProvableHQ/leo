program test.aleo {
    mapping balances: address => u64;

    fn deposit(amount: u64) -> Final {
        return finalize_deposit(self.caller, amount);
    }

    final fn finalize_deposit(caller: address, amount: u64) {
        balances.set(caller, amount);
    }
}
