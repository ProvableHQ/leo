// Safe: a `view fn` call is a Check; calling it BEFORE the interaction follows the
// Checks-Effects-Interactions order, so no warning should be emitted.
program dep.aleo {
    mapping shared: u32 => u32;

    fn modify(a: u32) -> Final {
        return final { finalize_modify(a); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_modify(a: u32) {
    Mapping::set(shared, 0u32, a);
}

// --- Next Program --- //
import dep.aleo;

program caller.aleo {
    mapping balances: u32 => u32;

    view fn balance_of(key: u32) -> u32 {
        return Mapping::get_or_use(balances, key, 0u32);
    }

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::modify(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f: Final, a: u32) {
    // [C] Check via view BEFORE any interaction — safe.
    let current: u32 = balance_of(0u32);
    // [E] Effect before interaction — safe.
    Mapping::set(balances, 0u32, current + a);
    // [I] Interaction last — correct CEI order.
    f.run();
}
