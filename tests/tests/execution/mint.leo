/*
namespace: Execute
expectation: Pass
cases:
    mint:
    - input: ["aleo14lskz87tkqwwkyt2z44h64ave5gcwqs6yyfdztus37nupxsj8ypsmqsqcs",  "0u64"]
    - input: ["aleo12keuztkg4cjzxx7hwwmrnnv85dkeqf8pjm877lf6f6lupma2pqrqcl2d8q", "1u64"]
*/


program test.aleo {
    record Token {
        // The token owner.
        private owner: address,
        // The token amount.
        amount: u64,
        // The flag.
        constant flag: bool,
    }

    transition mint(owner: address, amount: u64) -> Token {
        return Token {
            owner,
            amount: amount,
            flag: true,
        };
    }
}
