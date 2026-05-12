/*
seed = 123456789
min_height = 16

[case]
program = "test.aleo"
function = "cache_total"
input = ["aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9"]
private_key = "APrivateKey1zkpH5Ne1Xfd79t61VhK7b6yaYz92yW5dbuVkiFheR7rwCDE"
seed_mapping = ["balances", "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "42u64"]
*/

// End-to-end test for finalize-calls-query (snarkVM PR #3253). The
// entry-point's `final {}` block calls a same-program `query fn`,
// adds a constant, and writes the result into another mapping. The
// `balances` entry is seeded so the query returns a non-default value.
program test.aleo {
    mapping balances: address => u64;
    mapping totals:   address => u64;

    query fn get_balance(addr: address) -> u64 {
        return balances.get_or_use(addr, 0u64);
    }

    fn cache_total(addr: address) -> Final {
        return final {
            let bal: u64 = get_balance(addr);
            Mapping::set(totals, addr, bal + 100u64);
        };
    }

    @noupgrade
    constructor() {}
}
