program test.aleo {
    fn main(a: dyn, b: dyn, flag: bool) -> address {
        let r: dyn = flag ? a : b;
        return r.owner;
    }
}
