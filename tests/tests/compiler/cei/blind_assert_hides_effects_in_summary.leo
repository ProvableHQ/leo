// Control test: check hidden in helper behind assert.
// The assert statement itself sets has_checks=true in the effect summary,
// so this SHOULD be detected even though the mapping read is inside the assert.
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

final fn check_balance(key: u32) {
    assert(Mapping::get_or_use(balances, key, 0u32) > 0u32);
}

final fn finalize_do_work(f: Final, a: u32) {
    // [I] Interaction
    f.run();
    // [C] Check hidden in helper behind assert — summary HAS has_checks from assert
    check_balance(0u32);
}
