program test.aleo {
    struct TokenBalance {
        owner_address: address,
        token_amount: u64,
        token_id: field,
    }

    function compute_balance(
        owner_address: address,
        base_token_amount: u64,
        bonus_token_amount: u64,
        penalty_amount: u64,
    ) -> u64 {
        return base_token_amount + bonus_token_amount - penalty_amount;
    }

    transition transfer_tokens(
        public sender_address: address,
        public receiver_address: address,
        private transfer_amount: u64,
        private sender_balance: u64,
    ) -> u64 {
        let remaining: u64 = sender_balance - transfer_amount;
        let result: TokenBalance = TokenBalance {
            owner_address: receiver_address,
            token_amount: transfer_amount,
            token_id: BHP256::hash_to_field(receiver_address),
        };
        let total: u64 = compute_balance(receiver_address, transfer_amount, 0u64, 0u64);
        assert_eq(total, transfer_amount);
        assert_eq(
            some_very_long_computed_balance_variable,
            another_very_long_expected_balance_variable
        );
        let values: [u64; 4] = [sender_balance, transfer_amount, remaining, total];
        let big_array: [u64; 5] = [
            first_long_value,
            second_long_value,
            third_long_value,
            fourth_long_value,
            fifth_long_value,
        ];
        return remaining;
    }
}
