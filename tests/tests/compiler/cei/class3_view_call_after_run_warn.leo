// Class 3: Interact then Check — calling a same-program `view fn` after `f.run()`.
// The view reads state that may have been mutated by the interaction, which is the
// hidden-state-read pattern the CEI analyser exists to flag.
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
        return final { finalize_do_work(f); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f: Final) {
    // [I] Interaction: run external future (may mutate state).
    f.run();
    // [C] Check hidden in a view: stale read after the interaction.
    let val: u32 = balance_of(0u32);
    assert(val >= 0u32);
}
