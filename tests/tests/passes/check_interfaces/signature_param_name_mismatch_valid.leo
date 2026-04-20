// Test: parameter names do not need to match between an interface prototype and the
// implementing function. Only types, modes, order, and return type are significant.
// Regression test for https://github.com/ProvableHQ/leo/issues/29327.

interface Transferable {
    fn transfer(from: address, amount: u64) -> u64;
}

program test.aleo: Transferable {
    // Different parameter names (`sender` and `qty` vs `from` and `amount`) — must compile.
    fn transfer(sender: address, qty: u64) -> u64 {
        return qty;
    }

    fn main() {}
}
