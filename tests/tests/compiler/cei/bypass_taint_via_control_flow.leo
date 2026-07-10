// Cross-layer taint laundered through control-flow dependence.
//
// The tainted proof-time value `price` is NOT passed into the finalize block.
// Instead, it controls a transition-level conditional that assigns a CLEAN
// literal to `action`.  Because `price` is tainted and controls the branch,
// the implicit taint tracking marks `action` as tainted (control-flow
// dependence), and a warning is emitted.
//
// Attack scenario (detected):
//   1. At proof time the oracle returns price = 500 (above threshold).
//   2. The prover takes the "buy" branch → action = 1.
//   3. By the time finalize executes, the oracle's finalize updates the
//      on-chain price to 50 — well below the threshold.
//   4. The caller's finalize records action = 1 ("buy") even though the
//      current on-chain price no longer justifies the trade.
//
// Phase 2 (cross-layer taint) catches this via implicit taint propagation.
program oracle.aleo {
    mapping prices: u32 => u64;

    fn get_price(id: u32) -> (u64, Final) {
        let f: Final = final { finalize_get_price(id); };
        return (500u64, f);
    }

    @noupgrade
    constructor() {}
}

export final fn finalize_get_price(id: u32) {
    // The oracle's finalize might update the price, making the
    // proof-time snapshot stale.
    Mapping::set(prices, id, 50u64);
}

// --- Next Program --- //
import oracle.aleo;

program trader.aleo {
    mapping positions: u32 => u64;

    fn trade(id: u32) -> Final {
        // price is tainted (proof-time snapshot coupled to f).
        let (price, f): (u64, Final) = oracle.aleo::get_price(id);

        // Control-flow laundering: the tainted `price` determines the branch,
        // but `action` is assigned only clean literals.
        let action: u64 = 0u64;
        if (price > 400u64) {
            action = 1u64;   // "buy"
        }
        // `action` is semantically tainted (its value depends on stale `price`)
        // and the implicit taint tracking correctly detects this.

        return final {
            // action is used in finalize — taint warning emitted
            Mapping::set(positions, id, action);
            // Proper CEI order: effect before interaction — no Phase 1 warning!
            f.run();
        };
    }

    @noupgrade
    constructor() {}
}
