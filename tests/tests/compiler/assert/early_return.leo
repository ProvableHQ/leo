
// This test intends to make sure the assertion
// is not triggered if the early return is taken.
program test.aleo {
    fn main(a: bool) -> bool {
        if a {
            return a;
        }
        assert(false);
        return a;
    }
}
