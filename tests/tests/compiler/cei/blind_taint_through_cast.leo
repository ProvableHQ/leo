// Verify: taint propagation through a local wrapper function.
// A local function calls an external function and returns only the value part.
// The taint should still be tracked since the wrapper's return is derived from a tainted source.
program exchange.aleo {
    mapping rates: u32 => u64;

    fn get_rate(a: u32) -> (u64, Final) {
        return (0u64, final { finalize_get_rate(a); });
    }
}

final fn finalize_get_rate(a: u32) {
    Mapping::set(rates, 0u32, a as u64);
}

// --- Next Program --- //
import exchange.aleo;

program caller.aleo {
    mapping balances: u32 => u64;

    fn do_work(a: u32) -> Final {
        let (rate, f): (u64, Final) = exchange.aleo::get_rate(a);
        // rate is tainted, coupled to f
        // Derived value carries taint
        let adjusted: u64 = rate * 2u64;
        return final {
            // adjusted is derived from tainted rate — should warn
            Mapping::set(balances, 0u32, adjusted);
            f.run();
        };
    }
}
