// Two external calls produce independently tainted values that are
// combined before reaching finalize.  Tests that collect_taint merges taint
// from multiple independent sources and reports both coupled futures.
program dep_a.aleo {
    mapping data_a: u32 => u64;

    fn get_value(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_value(id); };
        return (10u64, f);
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_get_value(id: u32) {
    Mapping::set(data_a, id, 10u64);
}

// --- Next Program --- //
program dep_b.aleo {
    mapping data_b: u32 => u64;

    fn get_value(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_value(id); };
        return (20u64, f);
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_get_value(id: u32) {
    Mapping::set(data_b, id, 20u64);
}

// --- Next Program --- //
import dep_a.aleo;
import dep_b.aleo;

program caller.aleo {
    mapping results: u32 => u64;

    fn do_work(id: u32) -> Final {
        let (val_a, fa): (u64, Final) = dep_a.aleo::get_value(id);
        let (val_b, fb): (u64, Final) = dep_b.aleo::get_value(id);
        // Combine tainted values from TWO independent external calls
        let combined: u64 = val_a + val_b;
        return final {
            // combined carries taint from both val_a (coupled to fa) and val_b (coupled to fb)
            Mapping::set(results, 0u32, combined);
            fa.run();
            fb.run();
        };
    }

    @noupgrade
    constructor() {}
}
