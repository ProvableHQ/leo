struct TransferInfo {
    sender_amount: u64,
    receiver_amount: u64,
    transfer_fee: u64,
}

fn compute_remaining(
    sender_amount: u64,
    receiver_amount: u64,
    transfer_fee: u64,
    bonus_amount: u64,
) -> u64 {
    return sender_amount - receiver_amount - transfer_fee + bonus_amount;
}

program test.aleo {
    fn transfer(
        public sender_amount: u64,
        public receiver_amount: u64,
        private transfer_fee: u64,
        private is_valid: bool,
    ) -> u64 {
        let bonus_amount: u64 = sender_amount + 1u64;
        let remaining: u64 = compute_remaining(
            sender_amount,
            receiver_amount,
            transfer_fee,
            bonus_amount,
        );
        let info: TransferInfo = TransferInfo {
            sender_amount: sender_amount,
            receiver_amount: receiver_amount,
            transfer_fee: transfer_fee,
        };
        assert_eq(info.sender_amount, sender_amount);
        assert_eq(
            info.sender_amount + info.receiver_amount + info.transfer_fee,
            sender_amount + receiver_amount + transfer_fee
        );
        let amounts: [u64; 5] = [
            sender_amount,
            receiver_amount,
            transfer_fee,
            remaining,
            sender_amount + receiver_amount,
        ];
        return remaining;
    }
}
