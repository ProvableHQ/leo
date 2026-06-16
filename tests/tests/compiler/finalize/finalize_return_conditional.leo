// A `final fn` with returns embedded in conditionals: inlining must duplicate the
// caller's continuation into each return path (AVM finalize registers cannot merge
// values across branches).
program test.aleo {
    mapping account: address => u64;
    mapping audit: u8 => u64;

    fn mint_capped(public receiver: address, public amount: u64) -> Final {
        return final {
            let credited: u64 = capped_amount(receiver, amount);
            Mapping::set(account, receiver, credited);
            Mapping::set(audit, 0u8, credited);
        };
    }

    @noupgrade
    constructor() {}
}

final fn capped_amount(receiver: address, amount: u64) -> u64 {
    let current: u64 = Mapping::get_or_use(account, receiver, 0u64);
    if current + amount > 1000u64 {
        return 1000u64;
    }
    return current + amount;
}
