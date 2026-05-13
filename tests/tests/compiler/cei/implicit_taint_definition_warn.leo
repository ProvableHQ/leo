// `let x: u64 = 42u64;` inside a tainted branch (definition, not reassignment)
// should also be tainted.
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

        let action: u64 = 0u64;
        if (price > 400u64) {
            let x: u64 = 42u64;  // definition inside tainted branch
            action = x;
        }

        return final {
            Mapping::set(positions, id, action);
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}
