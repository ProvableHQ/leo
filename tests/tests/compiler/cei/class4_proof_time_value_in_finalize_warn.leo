// Class 4: Cross-layer staleness — proof-time snapshot value passed to finalize alongside its Future.
// The value was captured at proof time but may be stale by finalize time.
program token.aleo {
    mapping supply: u32 => u32;

    fn get_supply(id: u32) -> (u32, Final) {
        let f: Final = final { finalize_get_supply(id); };
        // At proof time, return a snapshot value
        return (100u32, f);
    }

    @noupgrade
    constructor() {}
}

final fn finalize_get_supply(id: u32) {
    let s: u32 = Mapping::get_or_use(supply, id, 0u32);
    Mapping::set(supply, id, s);
}

// --- Next Program --- //
import token.aleo;

program caller.aleo {
    mapping decisions: u32 => u32;

    fn make_decision(id: u32) -> Final {
        // [Cross-layer staleness] val is a proof-time snapshot coupled to f
        let (val, f): (u32, Final) = token.aleo::get_supply(id);
        // val is passed to finalize alongside the future
        return final { finalize_make_decision(f, val, id); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_make_decision(f: Final, val: u32, id: u32) {
    // [E] Effect: use proof-time val which may now be stale
    Mapping::set(decisions, id, val);
    // [I] Interaction: run the future (supply may have changed)
    f.run();
}
