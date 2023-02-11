/*
namespace: Execute
expectation: Pass
cases:
    mint:
    - input: ["0u64"]
    - input: ["1u64"]
*/


program test.aleo {
    record Token {
        // The token owner.
        private owner: address,
        // The Aleo balance (in gates).
        public gates: u64,
        // The token amount.
        amount: u64,
        // The flag.
        constant flag: bool,
    }

    transition mint(amount: u64) -> Token {
        return Token {
            owner: self.caller,
            gates: 0u64,
            amount: amount,
            flag: true,
        };
    }
}
