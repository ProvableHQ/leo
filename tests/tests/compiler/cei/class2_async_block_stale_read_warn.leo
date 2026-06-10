// Class 2: Check then Interact then Effect — stale read using inline async block + method syntax.
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
        return final {
            // [C] Check via method syntax
            let current: u32 = balances.get_or_use(0u32, 0u32);
            // [I] Interaction
            f.run();
            // [E] Effect using stale current
            balances.set(0u32, current + a);
        };
    }

    @noupgrade
    constructor() {}
}
