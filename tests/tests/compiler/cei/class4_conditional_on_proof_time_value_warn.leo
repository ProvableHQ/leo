// Class 4: Cross-layer staleness — branching in finalize based on a proof-time snapshot.
program oracle.aleo {
    mapping prices: u32 => u64;

    fn get_price(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_price(id); };
        // Proof-time snapshot of price
        return (500u64, f);
    }
}

final fn finalize_get_price(id: u32) {
    let p: u64 = Mapping::get_or_use(prices, id, 0u64);
    Mapping::set(prices, id, p);
}

// --- Next Program --- //
import oracle.aleo;

program trader.aleo {
    mapping positions: u32 => u64;

    fn trade(id: u32) -> Final {
        // [Cross-layer staleness] price is proof-time snapshot coupled to f
        let (price, f): (u64, Final) = oracle.aleo::get_price(id);
        return final { finalize_trade(f, price, id); };
    }
}

final fn finalize_trade(f: Final, price: u64, id: u32) {
    // [E] Branch on stale proof-time price
    if price > 400u64 {
        Mapping::set(positions, id, price);
    } else {
        Mapping::set(positions, id, 0u64);
    }
    // [I] Run the oracle future — price on-chain may have changed
    f.run();
}
