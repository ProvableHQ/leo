// Nested conditional returns in a `final fn`: every return path must get its own
// copy of the caller's continuation.
program test.aleo {
    mapping tiers: address => u8;
    mapping audit: u8 => u8;

    fn classify(public who: address, public amount: u64) -> Final {
        return final {
            let tier: u8 = tier_of(amount);
            Mapping::set(tiers, who, tier);
            Mapping::set(audit, tier, tier);
        };
    }

    @noupgrade
    constructor() {}
}

export final fn tier_of(amount: u64) -> u8 {
    if amount >= 1000u64 {
        if amount >= 1_000_000u64 {
            return 3u8;
        }
        return 2u8;
    } else if amount >= 10u64 {
        return 1u8;
    }
    return 0u8;
}
