
program test.aleo {
    fn main(s: bool, a: i16, b: i16, c: i16) -> bool {
        let r: i16 = s ? a : b;

        return r == c;
    }
}
