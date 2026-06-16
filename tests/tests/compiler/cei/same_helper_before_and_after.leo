// Same helper called BEFORE interaction (safe) and AFTER interaction (violation).
// Tests that the analysis is context-sensitive per call site, not per function.
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

// Helper that reads and writes — final fn can't return values, so it writes in-place.
final fn update_balance(key: u32, val: u32) {
    let old: u32 = Mapping::get_or_use(balances, key, 0u32);
    Mapping::set(balances, key, old + val);
}

final fn finalize_do_work(f: Final, a: u32) {
    // [C+E] Same helper called BEFORE interaction — safe, no warn
    update_balance(0u32, a);
    // [I] Interaction
    f.run();
    // [C+E] Same helper called AFTER interaction — should warn
    update_balance(1u32, a);
}
