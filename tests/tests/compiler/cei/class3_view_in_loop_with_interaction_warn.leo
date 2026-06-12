// Loop-level CEI violation: each iteration both runs an interaction (`Final.run()`)
// and queries a `view fn`. Even though a view is read-only, mixing it with an
// interaction inside a loop is the same hazard the loop check exists for —
// the view sees state that may have been mutated by an earlier iteration.
program dep.aleo {
    mapping shared: u32 => u32;

    fn modify(a: u32) -> Final {
        return final { finalize_modify(a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_modify(a: u32) {
    Mapping::set(shared, 0u32, a);
}

// --- Next Program --- //
import dep.aleo;

program caller.aleo {
    mapping balances: u32 => u32;

    view fn balance_of(key: u32) -> u32 {
        return Mapping::get_or_use(balances, key, 0u32);
    }

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::modify(a);
        return final { finalize_do_work(f); };
    }

    @noupgrade
    constructor() {}
}

final fn run_future(f: Final) {
    f.run();
}

final fn finalize_do_work(f: Final) {
    for i: u32 in 0u32..3u32 {
        // [I] Interaction hidden in helper.
        run_future(f);
        // [C] View call after the interaction — stale read each iteration.
        let val: u32 = balance_of(i);
        assert(val >= 0u32);
    }
}
