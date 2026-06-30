// Effect sandwiched between two interactions.
// Both the check and the effect should be flagged.
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
        let f1: Final = dep.aleo::update(a);
        let f2: Final = dep.aleo::update(a + 1u32);
        return final { finalize_do_work(f1, f2, a); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f1: Final, f2: Final, a: u32) {
    // [I] First interaction
    f1.run();
    // [C] Check between interactions — should warn
    let old: u32 = Mapping::get_or_use(balances, 0u32, 0u32);
    // [E] Effect between interactions — should warn
    Mapping::set(balances, 0u32, old + a);
    // [I] Second interaction
    f2.run();
}
