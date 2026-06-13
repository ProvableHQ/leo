// Multiple distinct CEI violations in one finalize function.
// Both an effect-after-interaction and a check-after-interaction should be reported.
program dep_a.aleo {
    mapping data: u32 => u32;

    fn do_a(a: u32) -> Final {
        return final { finalize_do_a(a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_do_a(a: u32) {
    Mapping::set(data, 0u32, a);
}

// --- Next Program --- //
program dep_b.aleo {
    mapping data: u32 => u32;

    fn do_b(a: u32) -> Final {
        return final { finalize_do_b(a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_do_b(a: u32) {
    Mapping::set(data, 0u32, a);
}

// --- Next Program --- //
import dep_a.aleo;
import dep_b.aleo;

program caller.aleo {
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        let f1: Final = dep_a.aleo::do_a(a);
        let f2: Final = dep_b.aleo::do_b(a);
        return final { finalize_do_work(f1, f2, a); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_do_work(f1: Final, f2: Final, a: u32) {
    f1.run();
    // Effect after interaction — violation 1
    Mapping::set(balances, 0u32, a);
    f2.run();
    // Check after interaction — violation 2
    let _v: u32 = Mapping::get_or_use(balances, 0u32, 0u32);
}
