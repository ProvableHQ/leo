program test.aleo {
    fn main(a: bool) -> bool {
        let b: bool = a || false;
        let c: bool = b && true;
        return c;
    }
}
