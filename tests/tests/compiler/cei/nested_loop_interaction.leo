// Nested loops: inner loop body contains an interaction, outer loop
// body contains an effect after the inner loop.
// Should warn about the inner loop CEI violation and the effect
// after the interaction-containing inner loop.
program dep.aleo {
    mapping dep_data: u32 => u32;

    fn update(a: u32) -> Final {
        return final { finalize_update(a); };
    }

    @noupgrade
    constructor() {}
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

    @noupgrade
    constructor() {}
}

final fn finalize_do_work(f: Final, a: u32) {
    for i: u32 in 0u32..2u32 {
        // Inner loop: interaction + effect in the same body
        for j: u32 in 0u32..2u32 {
            f.run();
            Mapping::set(counters, j, a);
        }
        // [E] Effect after inner loop that had interactions
        Mapping::set(counters, i, a);
    }
}
