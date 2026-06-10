interface CounterInterface {
    mapping counter: address => u64;
}

program test.aleo: CounterInterface {
    @noupgrade
    constructor() {}

    mapping counter: address => u64;

    fn main() {}
}
