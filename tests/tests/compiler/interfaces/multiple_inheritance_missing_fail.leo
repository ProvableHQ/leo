export interface Counter {
    fn increment() -> u64;
}

export interface Resettable {
    fn reset() -> u64;
}

program test.aleo: Counter + Resettable {
    fn increment() -> u64 {
        return 1u64;
    }
    // Missing reset from Resettable

    @noupgrade
    constructor() {}
}
