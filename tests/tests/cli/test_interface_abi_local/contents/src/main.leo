interface Adder {
    fn compute_sum(a: u64, b: u64) -> u64;
}

interface Doubler {
    fn compute_double(x: u64) -> u64;
}

program foo.aleo : Adder + Doubler {
    fn compute_sum(a: u64, b: u64) -> u64 {
        return a + b;
    }

    fn compute_double(x: u64) -> u64 {
        return x + x;
    }

    @noupgrade
    constructor() {}
}
