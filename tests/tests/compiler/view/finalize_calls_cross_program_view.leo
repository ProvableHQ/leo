// A `final {}` block in `caller.aleo` calls a `view fn` exposed by an
// imported `data.aleo` program (snarkVM PR #3253). Codegen emits a
// cross-program `call data.aleo/get_balance ... into ...` inside the
// caller's `finalize` body.
program data.aleo {
    mapping balances: address => u64;

    fn dummy(x: u32) -> u32 { return x; }

    view fn get_balance(addr: address) -> u64 {
        return balances.get_or_use(addr, 0u64);
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import data.aleo;

program caller.aleo {
    mapping totals: address => u64;

    fn cache_total(addr: address) -> Final {
        return final {
            let bal: u64 = data.aleo::get_balance(addr);
            Mapping::set(totals, addr, bal + 100u64);
        };
    }

    @noupgrade
    constructor() {}
}
