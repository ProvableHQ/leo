
program test.aleo {
    fn main(a: i128, b: i128) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}
