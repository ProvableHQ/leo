// A tainted value (coupled to an external future) flowing through a value-returning
// `final fn` helper must remain tainted: the conservative rule "tainted argument =>
// tainted return" covers helper calls.
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
        return final {
            // `price` is tainted (coupled to `f`); the helper's return inherits the taint.
            let doubled: u64 = double(price);
            Mapping::set(positions, id, doubled);
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}

final fn double(x: u64) -> u64 {
    return x * 2u64;
}
