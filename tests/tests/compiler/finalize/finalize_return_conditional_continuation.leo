// Conditionals interacting with the iterative-inline lowering in both directions:
// `classify` has a conditional *after* a conditional-return callsite (the conditional is
// part of the duplicated continuation), and `guarded` calls a conditional-return helper
// from *inside* a conditional (the continuation is only the rest of the inner block;
// control may rejoin at the outer conditional's end since no value crosses it).
program test.aleo {
    mapping account: address => u64;
    mapping audit: u8 => u64;

    fn classify(public who: address, public amount: u64) -> Final {
        return final {
            let credited: u64 = capped(who, amount);
            if credited == 1000u64 {
                Mapping::set(audit, 1u8, credited);
            } else {
                Mapping::set(audit, 0u8, credited);
            }
            Mapping::set(account, who, credited);
        };
    }

    fn guarded(public who: address, public amount: u64, public enabled: bool) -> Final {
        return final {
            if enabled {
                let credited: u64 = capped(who, amount);
                Mapping::set(account, who, credited);
            }
            Mapping::set(audit, 2u8, amount);
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
