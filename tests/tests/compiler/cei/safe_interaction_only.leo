// Safe: Finalize only runs an external future, no local state operations.
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
    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f: Final) {
    // [I] Only interaction, no local state ops
    f.run();
}
