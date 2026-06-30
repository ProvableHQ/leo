program test.aleo {
    fn main(r: dyn record) -> address {
        return inner(r);
    }

    @noupgrade
    constructor() {}
}

export fn inner(r: dyn record) -> address {
    return r.owner;
}
