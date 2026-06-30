// The trailing write is value-independent but reads `doubled`, which is defined
// inside the duplicated prefix and therefore branch-local after cloning. Def-closure
// must pull it into the duplication; sharing it would read a register the taken
// path never wrote.
program test.aleo {
    mapping account: address => u64;
    mapping audit: u8 => u64;

    fn pay(public who: address, public amount: u64) -> Final {
        return final {
            let credited: u64 = capped(who, amount);
            let doubled: u64 = amount * 2u64;
            Mapping::set(account, who, credited);
            Mapping::set(audit, 0u8, doubled);
        };
    }

    @noupgrade
    constructor() {}
}

export final fn capped(receiver: address, amount: u64) -> u64 {
    let current: u64 = Mapping::get_or_use(account, receiver, 0u64);
    if current + amount > 1000u64 {
        return 1000u64;
    }
    return current + amount;
}
