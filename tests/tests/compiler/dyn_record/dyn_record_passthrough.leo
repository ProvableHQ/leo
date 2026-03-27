program test.aleo {
    fn main(r: dyn) -> address {
        return inner(r);
    }
}

fn inner(r: dyn) -> address {
    return r.owner;
}
