// Stale proof-time value laundered through external call arguments.
//
// The tainted proof-time value `price` from oracle.aleo is passed as an
// argument to exchange.aleo::execute_trade() instead of into the caller's
// finalize block.  Phase 2 detects this via `warn_tainted_call_arguments`,
// which checks whether any argument to an external call (that returns a
// Future) carries taint from a prior external call.
//
// Attack scenario (detected):
//   1. At proof time, oracle returns price = 500 (the "current" snapshot).
//   2. The prover calls exchange::execute_trade(id, 500), baking the stale
//      price into the exchange's finalize arguments.
//   3. At finalize time, the oracle's finalize sets the on-chain price to 50.
//   4. The exchange's finalize records a trade at price = 500, even though
//      the on-chain price is now 50.
//
// Phase 2 emits a warning for the tainted argument.
program oracle.aleo {
    mapping prices: u32 => u64;

    fn get_price(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_price(id); };
        // Proof-time snapshot: the "current" price at proving time.
        return (500u64, f);
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_get_price(id: u32) {
    // The oracle's finalize may update the price, making the
    // proof-time snapshot stale by the time other finalizes run.
    Mapping::set(prices, id, 50u64);
}

// --- Next Program --- //
program exchange.aleo {
    mapping trades: u32 => u64;

    fn execute_trade(id: u32, amount: u64) -> Final {
        return final { finalize_execute_trade(id, amount); };
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_execute_trade(id: u32, amount: u64) {
    // Records the trade at the given amount.
    // If amount was derived from a stale price, this is incorrect.
    Mapping::set(trades, id, amount);
}

// --- Next Program --- //
import oracle.aleo;
import exchange.aleo;

program trader.aleo {
    fn trade(id: u32) -> Final {
        // price is tainted — proof-time snapshot coupled to f_oracle.
        let (price, f_oracle): (u64, Final) = oracle.aleo::get_price(id);

        // Stale price laundered through external call argument:
        // - price never enters trader's finalize block
        // - it is baked into exchange's finalize arguments at proof time
        // - Phase 2 detects tainted `price` passed to external call
        let f_exchange: Final = exchange.aleo::execute_trade(id, price);

        return final {
            // Only futures appear here — no tainted variables referenced.
            // Phase 1: interactions after interactions are fine.
            f_oracle.run();
            f_exchange.run();
        };
    }

    @noupgrade
    constructor() {}
}
