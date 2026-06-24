// Miscellaneous shapes of value-returning `final fn` helpers: tuple returns,
// chained helper calls, discarded results, and use from a constructor.
program test.aleo {
    mapping account: address => u64;
    mapping flags: u8 => bool;

    fn update(public receiver: address, public amount: u64) -> Final {
        return final {
            let (balance, flagged): (u64, bool) = audit_credit(receiver, amount);
            Mapping::set(account, receiver, balance);
            Mapping::set(flags, 0u8, flagged);
            // A discarded result is allowed.
            bump(receiver);
        };
    }

    @custom
    constructor() {
        let initial: u64 = base_supply();
        Mapping::set(account, std::ctx::addr(), initial);
    }
}

final fn audit_credit(receiver: address, amount: u64) -> (u64, bool) {
    let balance: u64 = checked_add(Mapping::get_or_use(account, receiver, 0u64), amount);
    return (balance, balance > 1000u64);
}

final fn checked_add(a: u64, b: u64) -> u64 {
    return a + b;
}

final fn bump(receiver: address) -> u64 {
    let next: u64 = Mapping::get_or_use(account, receiver, 0u64) + 1u64;
    Mapping::set(account, receiver, next);
    return next;
}

final fn base_supply() -> u64 {
    return 1_000_000u64;
}
