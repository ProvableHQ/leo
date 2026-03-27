program test.aleo {
    fn main(r: dyn record?) -> address {
        return r.owner;
    }
}
