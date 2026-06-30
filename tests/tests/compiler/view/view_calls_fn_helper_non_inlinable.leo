// Regression: a `view fn` calls a regular `fn` helper that is *also* called from a transition's
// `final {}` block. The optional inlining heuristics (call_count == 1, empty args, only
// empty-typed args) all fail for `helper`, so without force-inlining from the analysis pass the
// codegen would emit a `call` inside the Aleo `view capped_balance:` block — which snarkVM
// rejects at deploy. The analysis pass must therefore mark `helper` as `always_inline` once it
// sees the call from inside a view body, even though it is also reachable from a transition
// (which is normally enough to keep it as a standalone closure under the optional-inline rules).
export fn helper(raw: u64, cap: u64) -> u64 {
    if raw > cap {
        return cap;
    } else {
        return raw;
    }
}

program test.aleo {
    mapping balances: address => u64;

    view fn capped_balance(addr: address, cap: u64) -> u64 {
        let raw: u64 = balances.get_or_use(addr, 0u64);
        return helper(raw, cap);
    }

    fn bump(addr: address, cap: u64) -> Final {
        return final {
            let raw: u64 = Mapping::get_or_use(balances, addr, 0u64);
            let capped: u64 = helper(raw, cap);
            Mapping::set(balances, addr, capped);
        };
    }

    @noupgrade
    constructor() {}
}
