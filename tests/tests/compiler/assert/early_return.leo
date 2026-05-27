// This test intends to make sure the assertion
// is not triggered if the early return is taken.
program test.aleo {
    fn main(a: bool, b: bool) -> bool {
        if a {
            return b;
        }
        assert(b);
        return a;
    }
}
