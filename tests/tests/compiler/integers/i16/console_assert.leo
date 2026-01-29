
program test.aleo {
    fn main(a: i16, b: i16) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}
