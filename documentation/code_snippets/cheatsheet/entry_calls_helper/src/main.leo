// ANCHOR: entry_calls_helper
fn subtract(a: u64, b: u64) -> u64 {
    return a - b;
}

program example.aleo {
    fn transfer(receiver: address, amount: u64) {
        let balance: u64 = 1000u64;
        let new_balance: u64 = subtract(balance, amount);
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: entry_calls_helper
