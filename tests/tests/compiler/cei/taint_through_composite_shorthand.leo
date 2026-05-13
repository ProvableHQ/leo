// Tainted value laundered through a composite-init shorthand `Box { rate }`.
// Tests that collect_taint_recursive / check_expression_for_taint resolve
// shorthand fields (where `expression == None`) against the local taint map
// before later passes desugar them.
struct Box {
    rate: u64,
}

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

struct Box {
    rate: u64,
}

program caller.aleo {
    mapping balances: u32 => u64;

    fn do_work(id: u32) -> Final {
        let (rate, f): (u64, Final) = exchange.aleo::get_rate(id);
        // Shorthand initializer — `rate` carries taint from the external call,
        // and the taint must propagate into `b` even though the field has no
        // explicit value expression.
        let b: Box = Box { rate };
        return final {
            // Reading b.rate inside finalize must warn.
            Mapping::set(balances, 0u32, b.rate);
            f.run();
        };
    }
}
