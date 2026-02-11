
program test.aleo {
    fn main(a: i32, b: i32) -> bool {
        let ret: bool = a == b;
        assert(ret);
        return ret;
    }
}
