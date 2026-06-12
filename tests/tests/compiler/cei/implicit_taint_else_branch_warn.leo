// Assignment in the else branch of a tainted condition is also implicitly tainted.
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

    fn trade(id: u32) -> Final {
        let (price, f): (u64, Final) = oracle.aleo::get_price(id);

        let action: u64 = 0u64;
        if (price > 400u64) {
            // then branch — not taken path
        } else {
            action = 2u64;  // else branch — still tainted by condition
        }

        return final {
            Mapping::set(positions, id, action);
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}
