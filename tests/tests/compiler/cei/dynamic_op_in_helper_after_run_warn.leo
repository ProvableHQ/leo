// Dynamic mapping op hidden in a helper function, called after interaction — should warn.
// The helper uses a dynamic get, which contributes has_checks to its effect summary.
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

interface Bank {
    mapping balances: address => u64;
}

program caller.aleo {
    mapping results: address => u64;

    fn do_work(a: u32, target: field, key: address) -> Final {
        let f: Final = dep.aleo::update(a);
        return final { finalize_do_work(f, target, key); };
    }

    @noupgrade
    constructor() {}
}

// Helper that performs a dynamic check and stores the result.
final fn read_and_store(target: field, key: address) {
    let val: u64 = Bank@(target)::balances.get(key);
    Mapping::set(results, key, val);
}

final fn finalize_do_work(f: Final, target: field, key: address) {
    // [I] Interaction
    f.run();
    // [C+E] Helper with dynamic check + effect called after interaction — should warn
    read_and_store(target, key);
}
