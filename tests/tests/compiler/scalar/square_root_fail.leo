program test.aleo {
    @noupgrade
    constructor() {}
}

fn main(a: scalar) -> scalar {
    return a.square_root(); // square root not supported for scalar types.
}
