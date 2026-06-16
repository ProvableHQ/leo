// Edge: Effect always happens, f.run() is conditional, then more effects after.
// If the branch is taken, state is inconsistent; if not, the later effects proceed normally.
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
    // [E] Effect always happens
    Mapping::set(balances, 0u32, a);
    // [I] Interaction is conditional
    if a > 5u32 {
        f.run();
    }
    // [E] More effects after conditional interaction
    Mapping::set(balances, 1u32, a + 1u32);
}
