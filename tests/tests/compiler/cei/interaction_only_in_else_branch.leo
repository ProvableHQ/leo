// Interaction only in the else-branch of a conditional.
// Tests that the conservative merge propagates AfterInteraction even when
// the interaction is in the else-branch (not the then-branch).
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
        // Then-branch: no interaction, just a noop
        Mapping::set(balances, 99u32, 0u32);
    } else {
        // Else-branch: interaction happens here
        f.run();
    }
    // [E] Effect after conditional where one branch had interaction — should warn
    Mapping::set(balances, 0u32, a);
}
