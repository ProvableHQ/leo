program test.aleo {
    fn main(a: dyn record, b: dyn record, flag: bool) -> address {
        let r: dyn record = flag ? a : b;
        return r.owner;
    }
}
