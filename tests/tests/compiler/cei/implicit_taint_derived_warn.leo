// Variable assigned a clean literal inside a tainted branch, then used in
// arithmetic before passing to finalize. The derived value should also be tainted.
program oracle.aleo {
    mapping prices: u32 => u64;

    fn get_price(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_price(id); };
        return (500u64, f);
    }
}

final fn finalize_get_price(id: u32) {
    Mapping::set(prices, id, 50u64);
}

// --- Next Program --- //
import oracle.aleo;

program trader.aleo {
    mapping positions: u32 => u64;

    fn trade(id: u32) -> Final {
        let (price, f): (u64, Final) = oracle.aleo::get_price(id);

        let multiplier: u64 = 1u64;
        if (price > 400u64) {
            multiplier = 10u64;  // implicitly tainted
        }

        // derived from implicitly tainted value
        let final_amount: u64 = multiplier * 5u64;

        return final {
            Mapping::set(positions, id, final_amount);
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}
