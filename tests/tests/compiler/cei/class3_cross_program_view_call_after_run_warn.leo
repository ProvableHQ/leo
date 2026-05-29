// Class 3 across programs: after running an external future, the caller queries a
// `view fn` exposed by a *different* program. The view's reads still count as
// checks-after-interaction — they read on-chain state from the same finalize stack.
program dep.aleo {
    mapping shared: u32 => u32;

    view fn shared_value(key: u32) -> u32 {
        return Mapping::get_or_use(shared, key, 0u32);
    }

    fn modify(a: u32) -> Final {
        return final { finalize_modify(a); };
    }
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
        return final { finalize_do_work(f); };
    }
}

final fn finalize_do_work(f: Final) {
    // [I] Interaction: run external future.
    f.run();
    // [C] Cross-program view call after the interaction.
    let val: u32 = dep.aleo::shared_value(0u32);
    assert(val >= 0u32);
}
