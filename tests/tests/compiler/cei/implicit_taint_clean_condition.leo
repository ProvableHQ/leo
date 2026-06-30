// Branch condition depends only on a non-tainted input parameter.
// Assignments inside should NOT be tainted.
program oracle.aleo {
    mapping prices: u32 => u64;

    fn get_price(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_price(id); };
        return (500u64, f);
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_get_price(id: u32) {
    Mapping::set(prices, id, 50u64);
}

// --- Next Program --- //
import oracle.aleo;

program trader.aleo {
    mapping positions: u32 => u64;

    fn trade(id: u32, threshold: u64) -> Final {
        let (price, f): (u64, Final) = oracle.aleo::get_price(id);

        // Use price locally so it's not unused, but don't pass to finalize
        let local: u64 = price + 1u64;
        assert(local > 0u64);

        // Condition uses only the clean input parameter `threshold`
        let action: u64 = 0u64;
        if (threshold > 400u64) {
            action = 1u64;  // NOT tainted — condition is clean
        }

        return final {
            Mapping::set(positions, id, action);
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}
