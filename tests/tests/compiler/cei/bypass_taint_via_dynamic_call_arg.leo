// Stale proof-time value laundered through dynamic call arguments.
//
// Same scenario as bypass_taint_via_external_call_arg.leo, but the tainted
// value is passed through a dynamic call (interface dispatch) instead of a
// static external call. Phase 2 detects this via `warn_tainted_call_arguments`.
interface Oracle {
    fn get_price(id: u32) -> (u64, Final);
}

program oracle.aleo: Oracle {
    mapping prices: u32 => u64;

    fn get_price(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_price(id); };
        return (500u64, f);
    }

    @noupgrade
    constructor() {}
}

final fn finalize_get_price(id: u32) {
    Mapping::set(prices, id, 50u64);
}

// --- Next Program --- //
interface Exchange {
    fn execute_trade(id: u32, amount: u64) -> Final;
}

program exchange.aleo: Exchange {
    mapping trades: u32 => u64;

    fn execute_trade(id: u32, amount: u64) -> Final {
        return final { finalize_execute_trade(id, amount); };
    }

    @noupgrade
    constructor() {}
}

final fn finalize_execute_trade(id: u32, amount: u64) {
    Mapping::set(trades, id, amount);
}

// --- Next Program --- //
import oracle.aleo;
import exchange.aleo;

program trader.aleo {
    fn trade(id: u32, oracle_target: field, exchange_target: field) -> Final {
        // price is tainted — proof-time snapshot coupled to f_oracle.
        let (price, f_oracle): (u64, Final) = oracle.aleo::Oracle@(oracle_target)::get_price(id);

        // Stale price laundered through dynamic call argument.
        let f_exchange: Final = exchange.aleo::Exchange@(exchange_target)::execute_trade(id, price);

        return final {
            f_oracle.run();
            f_exchange.run();
        };
    }

    @noupgrade
    constructor() {}
}
