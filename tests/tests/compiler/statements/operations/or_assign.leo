program test.aleo {
    fn main(a: bool, k: bool) -> bool {
        let b: bool = a;
        b ||= k;

        return b;
    }
}
