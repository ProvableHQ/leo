// Verify: interaction through a transitive call chain, then direct effect.
// Tests that effect summaries propagate has_interactions through multiple levels.
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
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

final fn deep_run(f: Final) {
    f.run();
}

final fn mid_helper(f: Final) {
    deep_run(f);
}

final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction hidden behind two levels of helpers
    mid_helper(f);
    // [E] Effect after transitive interaction — should be caught
    Mapping::set(balances, 0u32, a);
}
