// ANCHOR: file
final fn decrement_balance(sender: address, amount: u64) {
    let current_amount: u64 = Mapping::get_or_use(account, sender, 0u64);
    Mapping::set(account, sender, current_amount - amount);
}

program transfer.aleo {
    record token {
        owner: address,
        amount: u64,
    }

    mapping account: address => u64;

    fn transfer_public_to_private(
        receiver: address,
        public amount: u64
    ) -> (token, Final) {
        let new: token = token {
            owner: receiver,
            amount,
        };

        let caller: address = self.caller;
        return (new, final {
            decrement_balance(caller, amount);
        });
    }

    fn burn(public amount: u64) -> Final {
        let caller: address = self.caller;
        return final {
            decrement_balance(caller, amount);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
