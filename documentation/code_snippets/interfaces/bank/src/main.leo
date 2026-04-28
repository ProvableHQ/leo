// ANCHOR: file
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
// ANCHOR_END: file
