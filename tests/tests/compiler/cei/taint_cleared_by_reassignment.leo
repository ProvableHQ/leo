// A tainted variable is unconditionally reassigned to a clean value
// before reaching the finalize block. No warning should be emitted.
program exchange.aleo {
    mapping rates: u32 => u64;

    fn get_rate(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_rate(id); };
        return (100u64, f);
    }

    @noupgrade
    constructor() {}
}

final fn finalize_get_rate(id: u32) {
    let r: u64 = Mapping::get_or_use(rates, id, 0u64);
    Mapping::set(rates, id, r);
}

// --- Next Program --- //
import exchange.aleo;

program caller.aleo {
    mapping balances: u32 => u64;

    fn do_work(id: u32) -> Final {
        let (rate, f): (u64, Final) = exchange.aleo::get_rate(id);
        // rate is tainted, but we unconditionally overwrite it
        let val: u64 = rate;
        val = 42u64;   // clears taint — val is now clean
        return final {
            Mapping::set(balances, 0u32, val);
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}
