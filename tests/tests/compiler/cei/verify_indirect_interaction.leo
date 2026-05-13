// Verify: interaction through a helper function, then direct effect.
// Tests that effect summaries correctly track has_interactions through callees.
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

final fn run_future(f: Final) {
    f.run();
}

final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction hidden in helper
    run_future(f);
    // [E] Effect after indirect interaction — should be caught via effect summary
    Mapping::set(balances, 0u32, a);
}
