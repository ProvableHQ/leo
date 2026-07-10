// Class 1: Effect before Interaction — effect in async block both before and after f.run().
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
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final {
            // [E] Effect before interaction
            balances.set(0u32, a);
            // [I] Interaction
            f.run();
            // [E] Effect after interaction — violates CEI
            balances.set(1u32, a);
        };
    }

    @noupgrade
    constructor() {}
}
