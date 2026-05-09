/*
[case]
program = "test.aleo"
function = "balance_of"
input = ["aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9"]

[case]
program = "test.aleo"
function = "balance_of"
input = ["aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9"]
seed_mapping = ["balances", "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "1234u64"]

[case]
program = "test.aleo"
function = "doubled"
input = ["7u64"]
*/

// Off-consensus query functions: read-only entry points returning plaintext.
// These tests exercise the `query`-block execution path through
// `Process::evaluate_query_at_height`.
//
// Note: query (and function) names cannot collide with Aleo opcode names
// (snarkVM's `add_query` rejects them), so we use `doubled` rather than
// `double` (the opcode).
program test.aleo {
    mapping balances: address => u64;

    fn dummy_transition(x: u32) -> u32 {
        // snarkVM rejects deployments with zero functions, so we keep one minimal transition.
        return x + 1u32;
    }

    // First call: mapping is empty, returns the default 0u64.
    // Second call: mapping has a seeded entry for the same address, returns the seeded value.
    query fn balance_of(addr: address) -> u64 {
        return balances.get_or_use(addr, 0u64);
    }

    query fn doubled(x: u64) -> u64 {
        return x + x;
    }

    @noupgrade
    constructor() {}
}
