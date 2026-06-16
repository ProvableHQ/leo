// Control flow: Both if/else branches have different CEI violations.
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

final fn finalize_do_work(f: Final, a: u32) {
    if a > 5u32 {
        // [C] Check
        let val: u32 = Mapping::get_or_use(balances, 0u32, 0u32);
        // [I] Interaction
        f.run();
        // [E] Effect with stale val — Class 2 violation
        Mapping::set(balances, 0u32, val + a);
    } else {
        // [E] Effect before interaction — Class 1 violation
        Mapping::set(balances, 1u32, a);
        // [I] Interaction
        f.run();
    }
}
