// ANCHOR: file
import data.aleo;

program cache.aleo {
    mapping totals: address => u64;

    // A `final {}` block in `cache.aleo` calls a `view fn` exposed by the
    // imported `data.aleo` program. Codegen emits a cross-program
    // `call data.aleo/get_balance ... into ...` inside the on-chain body.
    fn cache_total(account: address) -> Final {
        return final {
            let bal: u64 = data.aleo::get_balance(account);
            Mapping::set(totals, account, bal + 100u64);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
