// Test: Program missing a required interface record (should fail)

interface TokenStandard {
    record Token;
    fn transfer(amount: u64) -> u64;
}

program test.aleo : TokenStandard {
    // Missing Token record
    fn transfer(amount: u64) -> u64 {
        return amount;
    }

    fn main(x: u64) -> u64 {
        return transfer(x);
    }
}
