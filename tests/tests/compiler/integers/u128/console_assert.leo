
program test.aleo {
    fn main(a: u128, b: u128) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}
