// Regression test: taint must NOT be cleared by an else-branch.
// If the then-branch taints a variable and the else-branch assigns clean,
// the variable is still potentially tainted (depending on which branch runs).
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
        let val: u64 = 0u64;
        if id > 5u32 {
            val = rate;     // taints val
        } else {
            val = 42u64;    // must NOT clear taint — then-branch path is still possible
        }
        return final {
            Mapping::set(balances, 0u32, val);
            f.run();
        };
    }
}
