// Test: Interface inheritance with conflicting member signatures (should fail)

interface Base {
    fn process(x: u64) -> u64;
}

interface Derived : Base {
    // Conflicting: same name, different return type
    fn process(x: u64) -> u32;
}

program test.aleo {
    fn main(x: u64) -> u64 {
        return x;
    }
}
