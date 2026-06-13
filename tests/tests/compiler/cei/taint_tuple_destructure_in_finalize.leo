// Taint propagation through tuple destructuring inside the finalize body.
// A tainted variable flows into a tuple, and the destructured component should still warn.
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

    fn do_work(id: u32, amount: u64) -> Final {
        let (rate, f): (u64, Final) = exchange.aleo::get_rate(id);
        return final {
            // Destructure a tuple containing tainted value.
            let (a, _b): (u64, u64) = (rate, amount);
            // a derives from rate (tainted), b is clean.
            Mapping::set(balances, id, a);
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}
