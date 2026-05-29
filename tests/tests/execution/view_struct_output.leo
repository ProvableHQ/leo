/*
[case]
program = "test.aleo"
function = "account_summary"
input = ["aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9"]
seed_mapping = ["balances", "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "1000u64"]
seed_mapping = ["nonces", "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "7u64"]

[case]
program = "test.aleo"
function = "is_active"
input = ["aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9"]
seed_mapping = ["balances", "aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9", "1u64"]

[case]
program = "test.aleo"
function = "is_active"
input = ["aleo1qnr4dkkvkgfqph0vzc3y6z2eu975wnpz2925ntjccd5cfqxtyu8s7pyjh9"]
*/

// Views that read multiple mappings and return a struct (or boolean).
program test.aleo {
    mapping balances: address => u64;
    mapping nonces: address => u64;

    struct AccountSummary {
        addr: address,
        balance: u64,
        nonce: u64,
    }

    fn dummy(x: u32) -> u32 { return x; }

    // Reads two different mappings and combines them into a struct output.
    view fn account_summary(addr: address) -> AccountSummary {
        let b: u64 = balances.get_or_use(addr, 0u64);
        let n: u64 = nonces.get_or_use(addr, 0u64);
        return AccountSummary {
            addr: addr,
            balance: b,
            nonce: n,
        };
    }

    // Returns whether the address has any positive balance.
    view fn is_active(addr: address) -> bool {
        return balances.get_or_use(addr, 0u64) > 0u64;
    }

    @noupgrade
    constructor() {}
}
