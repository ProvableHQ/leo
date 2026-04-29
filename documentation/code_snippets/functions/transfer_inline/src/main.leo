// ANCHOR: file
program transfer.aleo {
    record token {
        owner: address,
        amount: u64,
    }

    mapping account: address => u64;

    // The function `transfer_public_to_private` turns a specified token amount
    // from `account` into a token record for the specified receiver.
    //
    // This function preserves privacy for the receiver's record, however
    // it publicly reveals the sender and the specified token amount.
    fn transfer_public_to_private(
        receiver: address,
        public amount: u64
    ) -> (token, Final) {
        // Produce a token record for the token receiver.
        let new: token = token {
            owner: receiver,
            amount,
        };

        let caller: address = self.caller;
        // Return the receiver's record, then decrement the token amount of the caller publicly.
        return (new, final {
            // Decrements `account[sender]` by `amount`.
            // If `account[sender]` does not exist, it will be created.
            // If `account[sender] - amount` underflows, `transfer_public_to_private` is reverted.
            let current_amount: u64 = Mapping::get_or_use(account, caller, 0u64);
            Mapping::set(account, caller, current_amount - amount);
        });
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
