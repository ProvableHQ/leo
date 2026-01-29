
program test.aleo {
    fn main(a: i64, b: i64) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}
