// Multiple storage variable writes after an interaction.
// Each write should produce a separate warning.
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
    storage flag: bool;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction first
    f.run();
    // [E] Effect: storage variable write after interaction — should warn
    counter = a;
    // [E] Effect: second storage variable write after interaction — should warn
    flag = true;
}
