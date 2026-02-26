interface Storage1 {
    storage value: u64;
}

interface Storage2 {
    storage value: u32;
}

interface Combined : Storage1 + Storage2 {}

program test.aleo : Combined {
    fn main() {}
}
