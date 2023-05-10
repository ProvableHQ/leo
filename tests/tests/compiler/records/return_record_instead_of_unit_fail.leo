/*
namespace: Compile
expectation: Fail
*/

program test.aleo {
    record test_credits {
        owner: address,
        amount: u64
    }

    transition mint_credits(to: address, amount: u64) {
        return test_credits {
            owner: self.caller,
            amount
        };
    }
}
