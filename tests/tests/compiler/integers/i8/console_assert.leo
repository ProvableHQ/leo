
program test.aleo {
    fn main(a: i8, b: i8) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}
