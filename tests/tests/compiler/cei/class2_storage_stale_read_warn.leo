// Class 2: Check then Interact then Effect — stale storage variable read.
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
    storage counter: u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_do_work(f: Final, a: u32) {
    // [C] Check: read storage value
    let old_val: u32 = counter.unwrap_or(0u32);
    // [I] Interaction: run external future
    f.run();
    // [E] Effect: write based on stale old_val
    counter = old_val + a;
}
