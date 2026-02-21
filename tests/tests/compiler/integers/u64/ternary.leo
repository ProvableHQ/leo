
program test.aleo {
    fn main(s: bool, a: u64, b: u64, c: u64) -> bool {
        let r: u64 = s ? a : b;

        return r == c;
    }
}
