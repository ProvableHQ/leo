// Multiple futures: state write interleaved between two f.run() calls.
program dep_a.aleo {
    mapping data_a: u32 => u32;

    fn update(a: u32) -> Final {
        return final { finalize_update(a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_update(a: u32) {
    Mapping::set(data_a, 0u32, a);
}

// --- Next Program --- //
program dep_b.aleo {
    mapping data_b: u32 => u32;

    fn update(a: u32) -> Final {
        return final { finalize_update(a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_update(a: u32) {
    Mapping::set(data_b, 0u32, a);
}

// --- Next Program --- //
import dep_a.aleo;
import dep_b.aleo;

program caller.aleo {
    mapping results: u32 => u32;

    fn do_work(a: u32) -> Final {
        let fa: Final = dep_a.aleo::update(a);
        let fb: Final = dep_b.aleo::update(a + 1u32);
        return final { finalize_do_work(fa, fb, a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_do_work(fa: Final, fb: Final, a: u32) {
    // [I] First interaction
    fa.run();
    // [E] Effect between two interactions
    Mapping::set(results, 0u32, a);
    // [I] Second interaction — state was modified between the two
    fb.run();
}
