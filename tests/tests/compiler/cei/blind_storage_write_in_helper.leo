// Storage variable write in a helper function called after interaction.
// The effect summary correctly tracks storage variable writes via `peel_assign_root`,
// so the callee check at the call site detects that the helper mutates state.
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
    storage counter: u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

export final fn write_counter(val: u32) {
    counter = val;
}

export final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction
    f.run();
    // [E] Effect hidden in helper — detected via effect summary
    write_counter(a);
}
