// ANCHOR: file
import bank.aleo;

program checker.aleo {
    mapping snapshot: address => u64;

    fn read_balance(target: field, user: address) -> Final {
        return final { do_read(target, user); };
    }

    @noupgrade
    constructor() {}
}

final fn do_read(target: field, user: address) {
    let present: bool = bank.aleo::Bank@(target)::balances.contains(user);
    let val: u64 = bank.aleo::Bank@(target)::balances.get_or_use(user, 0u64);
    Mapping::set(snapshot, user, present ? val : 0u64);
}
// ANCHOR_END: file
