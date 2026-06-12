// A conditional-return helper called from another `final fn` that also returns
// conditionally: the inner weave result must itself weave correctly into the
// outer caller's continuation.
program test.aleo {
    mapping account: address => u64;
    mapping audit: u8 => u64;

    fn settle(public receiver: address, public amount: u64) -> Final {
        return final {
            let credited: u64 = settle_amount(receiver, amount);
            Mapping::set(account, receiver, credited);
            Mapping::set(audit, 0u8, credited);
        };
    }
}

final fn settle_amount(receiver: address, amount: u64) -> u64 {
    let capped: u64 = capped_amount(receiver, amount);
    if capped == 0u64 {
        return 1u64;
    }
    return capped;
}

final fn capped_amount(receiver: address, amount: u64) -> u64 {
    let current: u64 = Mapping::get_or_use(account, receiver, 0u64);
    if current + amount > 1000u64 {
        return 1000u64;
    }
    return current + amount;
}
