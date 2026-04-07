// Interaction hidden behind TWO levels of call indirection.
// Tests that effect summaries propagate transitively through a→b→f.run().
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

// Depth 2: calls depth 1
final fn depth2(f: Final) {
    depth1(f);
}

// Depth 1: calls f.run() directly
final fn depth1(f: Final) {
    f.run();
}

final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction hidden two calls deep
    depth2(f);
    // [E] Effect after transitive interaction — should warn
    Mapping::set(balances, 0u32, a);
}
