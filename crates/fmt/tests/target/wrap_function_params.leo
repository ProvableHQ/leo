program test.aleo {
    fn add(a: u64, b: u64) -> u64 {
        return a + b;
    }

    fn transfer_tokens(
        public sender_balance: u64,
        public receiver_balance: u64,
        private transfer_amount: u64,
    ) -> u64 {
        let remaining: u64 = sender_balance - transfer_amount;
        return remaining;
    }
}
