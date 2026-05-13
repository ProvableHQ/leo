// Class 4: Cross-layer staleness — taint propagation through derived values.
// Even `rate * 2` is still tainted because it derives from the proof-time snapshot.
program exchange.aleo {
    mapping rates: u32 => u64;

    fn get_rate(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_rate(id); };
        return (100u64, f);
    }
}

final fn finalize_get_rate(id: u32) {
    let r: u64 = Mapping::get_or_use(rates, id, 0u64);
    Mapping::set(rates, id, r);
}

// --- Next Program --- //
import exchange.aleo;

program caller.aleo {
    mapping balances: u32 => u64;

    fn transfer(id: u32, amount: u64) -> Final {
        // [Cross-layer staleness] rate is proof-time snapshot
        let (rate, f): (u64, Final) = exchange.aleo::get_rate(id);
        // Derived value — still tainted
        let adjusted_rate: u64 = rate * 2u64;
        // Non-tainted value
        let local_val: u64 = amount * 2u64;
        return final { finalize_transfer(f, adjusted_rate, local_val, id); };
    }
}

final fn finalize_transfer(f: Final, adjusted_rate: u64, local_val: u64, id: u32) {
    // [E] Effect using tainted adjusted_rate — should warn
    Mapping::set(balances, id, adjusted_rate + local_val);
    f.run();
}
