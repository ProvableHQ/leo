// Test: interface requires a `public` input parameter but the implementing function
// uses the default (private) mode. `None` and `Private` are equivalent to each other
// but not to `Public`, so this must fail with a signature mismatch.

interface Transferable {
    fn transfer(public amount: u64) -> u64;
}

program test.aleo: Transferable {
    // Wrong mode: `amount` is private (default) but the interface requires public.
    fn transfer(amount: u64) -> u64 {
        return amount;
    }

    fn main(x: u64) -> u64 {
        return transfer(x);
    }
}
