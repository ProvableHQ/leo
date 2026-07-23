// Interaction buried at the bottom of 4 levels of nested conditionals.
// Tests that conservative merge propagates AfterInteraction upward through
// many nesting levels so the effect at the outermost level is caught.
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
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, a); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_do_work(f: Final, a: u32) {
    // 4 levels of nesting: interaction only fires in the deepest branch
    if a > 100u32 {
        if a > 200u32 {
            if a > 300u32 {
                if a > 400u32 {
                    // [I] Interaction at depth 4
                    f.run();
                } else {
                    Mapping::set(balances, 99u32, 0u32);
                }
            } else {
                Mapping::set(balances, 98u32, 0u32);
            }
        } else {
            Mapping::set(balances, 97u32, 0u32);
        }
    } else {
        Mapping::set(balances, 96u32, 0u32);
    }
    // [E] Effect after the deeply nested conditional — should warn
    Mapping::set(balances, 0u32, a);
}
