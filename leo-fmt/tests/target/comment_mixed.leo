// File-level comment
program test.aleo {
    function example(a: u64, b: u64) -> u64 {
        let x: u64 = a + b; // trailing comment
        // standalone comment
        let y: u64 = x * 2u64;
        // another standalone
        return y; // return value
    }
}
