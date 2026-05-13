// A helper final fn with effects is called after an interaction,
// but if the helper's Location doesn't match what's in effect_summaries
// (e.g., different program resolution), the callee check silently skips it.
// This tests that cross-function CEI through helpers works correctly.
program dep.aleo {
    mapping dep_data: u32 => u32;

    fn update(a: u32) -> Final {
        return final { finalize_update(a); };
    }
}

final fn finalize_update(a: u32) {
    Mapping::set(dep_data, 0u32, a);
}

// --- Next Program --- //
import dep.aleo;

program caller.aleo {
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }
}

// Helper that performs both a check and an effect.
final fn do_state_ops(val: u32) {
    let old: u32 = Mapping::get_or_use(balances, 0u32, 0u32);
    Mapping::set(balances, 0u32, old + val);
}

final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction first
    f.run();
    // [C+E] Helper with checks and effects called after interaction — should warn
    do_state_ops(a);
}
