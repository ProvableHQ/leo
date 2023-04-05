/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    record test_credits {
        owner: address,
        gates: u64,
        amount: u64
    }

    transition mint_credits(to: address, amount: u64) {
        return test_credits {
            owner: self.caller,
            gates: 0u64,
            amount
        };
    }
}