export interface Counter1 {
    mapping counter: address => u64;
}

export interface Counter2 {
    mapping counter: address => u32;
}

export interface Combined: Counter1 + Counter2 {
}

program test.aleo: Combined {
    fn main() {}

    @noupgrade
    constructor() {}
}
