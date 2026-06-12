// A value-returning `final fn` helper that reads state, interacts, and returns the
// pre-interaction read. The caller's write of the returned (stale) value happens
// after the interaction and must still be flagged.
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
    // [C] check, then [I] interaction inside the helper; the returned value is stale.
    let stale: u32 = read_then_interact(f);
    // [E] effect after the helper's interaction — must warn.
    Mapping::set(balances, 0u32, stale);
}

final fn read_then_interact(f: Final) -> u32 {
    let v: u32 = Mapping::get_or_use(balances, 0u32, 0u32);
    f.run();
    return v;
}
