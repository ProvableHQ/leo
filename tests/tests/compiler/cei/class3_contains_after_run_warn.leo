// Class 3: Interact then Check — Mapping::contains after f.run().
program dep.aleo {
    mapping dep_data: u32 => u32;

    fn update(a: u32) -> Final {
        return final { finalize_update(a); };
    }
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
}

final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction
    f.run();
    // [C] Check after interaction
    let exists: bool = Mapping::contains(balances, 0u32);
    if exists {
        // [E] Effect based on post-interaction check
        Mapping::set(balances, 0u32, a);
    }
}
