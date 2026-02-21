
program test.aleo {
    fn main(a: u32, b: u32) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}
