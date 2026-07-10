// Class 3: Interact then Check — reading external program's storage after f.run().
program dep.aleo {
    storage dep_counter: u32;

    fn bump() -> Final {
        return final { finalize_bump(); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_bump() {
    let c: u32 = dep_counter.unwrap_or(0u32);
    dep_counter = c + 1u32;
}

// --- Next Program --- //
import dep.aleo;

program caller.aleo {
    storage my_counter: u32;

    fn do_work() -> Final {
        let f: Final = dep.aleo::bump();
        return final { finalize_do_work(f); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f: Final) {
    // [I] Interaction: run external future that modifies dep_counter
    f.run();
    // [C] Check: read external storage after interaction — value changed
    let dep_val: u32 = dep.aleo::dep_counter.unwrap_or(0u32);
    // [E] Effect: use post-interaction value
    my_counter = dep_val;
}
