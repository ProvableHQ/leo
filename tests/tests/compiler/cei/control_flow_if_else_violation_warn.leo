// Control flow: CEI violation inside one branch of if/else.
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
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f: Final, a: u32) {
    if a > 10u32 {
        // [E] Effect in branch before interaction
        Mapping::set(balances, 0u32, a);
        // [I] Interaction in same branch after effect
        f.run();
    } else {
        // [I] Interaction first — then effect (also a violation: C/E after I)
        f.run();
        Mapping::set(balances, 0u32, a + 1u32);
    }
}
