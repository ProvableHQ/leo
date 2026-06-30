// Dynamic storage ops before interaction — should NOT warn.
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

export interface Bank {
    mapping balances: address => u64;
    storage total: u64;
}

program caller.aleo {
    mapping results: u32 => u64;

    fn do_work(a: u32, target: field, key: address) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, target, key); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f: Final, target: field, key: address) {
    // [C] All dynamic ops before interaction — safe
    let val: u64 = Bank@(target)::balances.get(key);
    let exists: bool = Bank@(target)::balances.contains(key);
    let total: u64? = Bank@(target)::total;
    // [I] Interaction after all checks
    f.run();
}
