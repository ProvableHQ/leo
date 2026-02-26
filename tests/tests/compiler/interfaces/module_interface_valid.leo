program test.aleo : interfaces::Counter {
    fn increment() -> u64 {
        return 1u64;
    }

    fn main(x: u64) -> u64 {
        return 0;
    }
}

// --- Next Module: interfaces.leo --- //

interface Counter {
    fn increment() -> u64;
}
