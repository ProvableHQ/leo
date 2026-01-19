
program test.aleo {
    fn main(a: bool, b: bool) -> (bool, bool) {
        let t: (bool, u64) = (a, b); // We should expect a bool, not a u64.

        return (t.0, t.1);
    }
}
