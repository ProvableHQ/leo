// Tainted value hidden inside one branch of a ternary expression.
// Tests that collect_taint_recursive visits all three arms of Ternary.
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
        // Taint laundered through ternary — only one branch is tainted
        let val: u64 = id > 5u32 ? rate : 0u64;
        return final {
            // val is conservatively tainted because one ternary branch uses rate
            Mapping::set(balances, 0u32, val);
            f.run();
        };
    }
}
