// Class 2: Check then Interact then Effect — stale mapping read.
// Read mapping value, run external future that may modify state,
// then write back based on the now-stale value.
program dep.aleo {
    mapping shared: u32 => u32;

    fn modify_shared(a: u32) -> Final {
        return final { finalize_modify_shared(a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_modify_shared(a: u32) {
    Mapping::set(shared, 0u32, a);
}

// --- Next Program --- //
import dep.aleo;

program caller.aleo {
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::modify_shared(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_do_work(f: Final, a: u32) {
    // [C] Check: read current value
    let current: u32 = Mapping::get_or_use(balances, 0u32, 0u32);
    // [I] Interaction: external call may change state
    f.run();
    // [E] Effect: write based on stale 'current' value
    Mapping::set(balances, 0u32, current + a);
}
