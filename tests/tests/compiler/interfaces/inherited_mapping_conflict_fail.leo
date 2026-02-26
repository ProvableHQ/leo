interface Counter1 {
    mapping counter: address => u64;
}

interface Counter2 {
    mapping counter: address => u32;
}

interface Combined : Counter1 + Counter2 {}

program test.aleo : Combined {
    fn main() {}
}
