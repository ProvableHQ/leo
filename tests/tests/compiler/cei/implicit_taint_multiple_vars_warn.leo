// Multiple variables assigned inside the same tainted branch should all be
// tainted independently.
program oracle.aleo {
    mapping prices: u32 => u64;

    fn get_price(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_price(id); };
        return (500u64, f);
    }

    @noupgrade
    constructor() {}
}

final fn finalize_get_price(id: u32) {
    Mapping::set(prices, id, 50u64);
}

// --- Next Program --- //
import oracle.aleo;

program trader.aleo {
    mapping positions: u32 => u64;
    mapping flags: u32 => u64;

    fn trade(id: u32) -> Final {
        let (price, f): (u64, Final) = oracle.aleo::get_price(id);

        let action: u64 = 0u64;
        let flag: u64 = 0u64;
        if (price > 400u64) {
            action = 1u64;  // implicitly tainted
            flag = 99u64;   // also implicitly tainted
        }

        return final {
            Mapping::set(positions, id, action);
            Mapping::set(flags, id, flag);
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}
