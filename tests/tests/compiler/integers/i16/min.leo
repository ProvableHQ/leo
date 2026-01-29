
program test.aleo {
    fn main(a: i16) -> i16 {
        let b: i16 = -32767i16;
        return b - a; // -32768i16
    }
}
