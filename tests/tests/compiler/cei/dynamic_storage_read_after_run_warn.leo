// Dynamic singleton storage read (DynamicOpKind::Read) after interaction — should warn.
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

export interface Counter {
    storage total: u64;
}

program caller.aleo {
    mapping results: u32 => u64;

    fn do_work(a: u32, target: field) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, target); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f: Final, target: field) {
    // [I] Interaction
    f.run();
    // [C] Dynamic singleton read after interaction — should warn
    let val: u64? = Counter@(target)::total;
}
