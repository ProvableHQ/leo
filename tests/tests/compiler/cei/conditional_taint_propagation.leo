// Taint is assigned to a variable inside a conditional body.
// Tests that the taint visitor's default visit_conditional walks inner
// statements (specifically AssignStatements) for taint propagation.
program exchange.aleo {
    mapping rates: u32 => u64;

    fn get_rate(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_rate(id); };
        return (100u64, f);
    }
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
        // val starts clean
        let val: u64 = 0u64;
        if id > 5u32 {
            // Taint sneaks in through conditional assignment
            val = rate;
        }
        // val is now (conditionally) tainted
        return final {
            Mapping::set(balances, 0u32, val);
            f.run();
        };
    }
}
