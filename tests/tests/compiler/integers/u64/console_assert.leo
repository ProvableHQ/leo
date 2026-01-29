
program test.aleo {
    fn main(a: u64, b: u64) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}
