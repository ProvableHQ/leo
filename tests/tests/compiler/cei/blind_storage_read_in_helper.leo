// Blind spot test: storage variable read in a helper function called after interaction.
// The effect summary now tracks storage variable reads via Path detection.
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
    storage counter: u32;
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f); };
    }

    @noupgrade
    constructor() {}
}

final fn read_and_use_counter() {
    // [C] Check: storage variable read, then use it in a mapping write
    let x: u32 = counter.unwrap_or(0u32);
    Mapping::set(balances, 0u32, x);
}

final fn finalize_do_work(f: Final) {
    // [I] Interaction
    f.run();
    // [C+E] Check and effect hidden in helper
    read_and_use_counter();
}
