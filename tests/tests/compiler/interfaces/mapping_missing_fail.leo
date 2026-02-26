interface CounterInterface {
    mapping counter: address => u64;
}

program test.aleo : CounterInterface {
    fn main() {}
}
