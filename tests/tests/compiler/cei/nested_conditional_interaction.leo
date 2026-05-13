// Interaction buried inside nested conditionals.
// The conservative merge should propagate AfterInteraction outward.
program dep.aleo {
    mapping dep_data: u32 => u32;

    fn update(a: u32) -> Final {
        return final { finalize_update(a); };
    }
}

final fn finalize_update(a: u32) {
    Mapping::set(dep_data, 0u32, a);
}

// --- Next Program --- //
import dep.aleo;

program caller.aleo {
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }
}

final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction hidden deep in nested conditionals
    if a > 0u32 {
        if a > 10u32 {
            if a > 100u32 {
                f.run();
            }
        }
    }
    // [E] Effect after nested conditional — should warn since some path has interaction
    Mapping::set(balances, 0u32, a);
}
