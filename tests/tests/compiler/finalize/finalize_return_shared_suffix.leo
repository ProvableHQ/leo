// Only the value-dependent prefix of the continuation is duplicated into the
// helper's return paths; the trailing write uses a pre-call value and must appear
// exactly once, after the join.
program test.aleo {
    mapping account: address => u64;
    mapping audit: u8 => u64;

    fn pay(public who: address, public amount: u64) -> Final {
        return final {
            let credited: u64 = capped(who, amount);
            Mapping::set(account, who, credited);
            Mapping::set(audit, 0u8, amount);
        };
    }

    @noupgrade
    constructor() {}
}

final fn capped(receiver: address, amount: u64) -> u64 {
    let current: u64 = Mapping::get_or_use(account, receiver, 0u64);
    if current + amount > 1000u64 {
        return 1000u64;
    }
    return current + amount;
}
