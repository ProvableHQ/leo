// Safe: All checks and effects happen before any interaction — correct CEI order.
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
    // [C] Check
    let current: u32 = Mapping::get_or_use(balances, 0u32, 0u32);
    // [E] Effect — all state writes before interaction
    Mapping::set(balances, 0u32, current + a);
    Mapping::set(balances, 1u32, a);
    // [I] Interaction last — correct CEI order
    f.run();
}
