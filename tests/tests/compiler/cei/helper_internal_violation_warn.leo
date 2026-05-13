// Helper final fn whose body internally violates CEI (run-then-effect).
// The calling finalize is otherwise clean; the violation lives entirely
// inside the helper.  The per-function walk of the helper should flag the
// effect at the helper's own body, not at the call site.
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

// Helper with internal CEI violation: runs the future, then writes state.
final fn buggy_helper(f: Final, key: u32, val: u32) {
    // [I] Interaction
    f.run();
    // [E] Effect after interaction — violation inside this helper
    Mapping::set(balances, key, val);
}

final fn finalize_do_work(f: Final, a: u32) {
    // Call the helper. The helper internally violates CEI.
    // The warning should surface at Mapping::set inside buggy_helper,
    // not at this call site.
    buggy_helper(f, 0u32, a);
}
