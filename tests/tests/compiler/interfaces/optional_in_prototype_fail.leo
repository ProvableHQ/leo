// Test: optional types are not allowed in interface function prototypes.
// Interfaces define program-level contracts; optional types are an internal
// Leo abstraction that should not appear in an interface declaration.

interface Finder {
    fn find(key: u32) -> u32?;
}

program test.aleo: Finder {
    fn find(key: u32) -> u32? {
        let result: u32? = 42u32;
        return result;
    }

    fn main(key: u32) -> u32 {
        return find(key).unwrap();
    }
}
