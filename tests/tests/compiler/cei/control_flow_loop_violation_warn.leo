// Control flow: for loop writes to mapping before f.run().
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
    mapping counters: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final {
            for i: u32 in 0u32..3u32 {
                // [E] Effect: loop writes state before interaction
                counters.set(i, a + i);

                // [I] Interaction after loop that wrote state
                f.run();
            }
        };
    }

    @noupgrade
    constructor() {}
}
