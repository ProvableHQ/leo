// Tainted value flows through a chain of arithmetic operations before reaching finalize.
// collect_taint conservatively propagates taint through binary operations.
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
        // Tainted rate laundered through multiple derivations
        let step1: u64 = rate * 2u64;
        let step2: u64 = step1 + 50u64;
        let step3: u64 = step2 / 3u64;
        return final {
            // step3 is transitively derived from tainted rate — should warn
            Mapping::set(balances, 0u32, step3);
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}
