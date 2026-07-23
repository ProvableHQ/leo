// Class 1: Effect after Interaction — vector mutation after f.run().
program dep.aleo {
    mapping dep_data: u32 => u32;

    fn update(a: u32) -> Final {
        return final { finalize_update(a); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_update(a: u32) {
    Mapping::set(dep_data, 0u32, a);
}

// --- Next Program --- //
import dep.aleo;

program caller.aleo {
    storage items: [u32];

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction
    f.run();
    // [E] Effect: push to vector AFTER interaction — violates CEI
    items.push(a);
}
