program test.aleo : interfaces::Counter {
    fn increment() -> u64 {
        return 1u64;
    }
}

// --- Next Module: interfaces.leo --- //

interface Counter {
    fn increment() -> u64;
}
