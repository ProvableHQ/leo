// ANCHOR: file
interface Counter {
    fn increment(amount: u64) -> u64;
}

program my_counter.aleo : Counter {
    fn increment(amount: u64) -> u64 {
        return amount + 1u64;
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
