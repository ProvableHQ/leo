interface Bank {
    mapping balances: address => u64;
}

program bank.aleo: Bank {
    mapping balances: address => u64;

    fn deposit(user: address, amount: u64) -> Final {
        return final { do_deposit(user, amount); };
    }

    @noupgrade
    constructor() {}
}

final fn do_deposit(user: address, amount: u64) {
    let prev: u64 = Mapping::get_or_use(balances, user, 0u64);
    Mapping::set(balances, user, prev + amount);
}

// --- Next Program --- //
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
