// Interaction hidden inside a helper that's called from a loop body.
// The loop body has both effects (Mapping::set) and an interaction (via helper).
// The loop-level check should detect this, AND the per-statement automaton
// should flag the effect after the helper-via-interaction.
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
    mapping counters: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }
}

final fn run_future(f: Final) {
    f.run();
}

final fn finalize_do_work(f: Final, a: u32) {
    for i: u32 in 0u32..3u32 {
        // [I] Interaction hidden in helper
        run_future(f);
        // [E] Effect after interaction-via-helper
        Mapping::set(counters, i, a + i);
    }
}
