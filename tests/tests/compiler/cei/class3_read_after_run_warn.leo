// Class 3: Interact then Check — reading state after f.run() without awareness it may have changed.
program dep.aleo {
    mapping shared: u32 => u32;

    fn modify(a: u32) -> Final {
        return final { finalize_modify(a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_modify(a: u32) {
    Mapping::set(shared, 0u32, a);
}

// --- Next Program --- //
import dep.aleo;

program caller.aleo {
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::modify(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_do_work(f: Final, _a: u32) {
    // [I] Interaction: run external future (modifies state)
    f.run();
    // [C] Check: read state that may have been changed by f.run()
    let val: u32 = Mapping::get_or_use(balances, 0u32, 0u32);
    assert(val >= 0u32);
}
