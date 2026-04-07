// Edge: Multiple storage types (mapping + storage var + vector) all written after f.run().
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
    storage counter: u32;
    storage items: [u32];

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction
    f.run();
    // [E] Multiple effects across different storage types AFTER interaction — violates CEI
    Mapping::set(balances, 0u32, a);
    counter = a;
    items.push(a);
}
