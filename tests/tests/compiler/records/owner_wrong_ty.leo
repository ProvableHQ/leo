program test.aleo {
    // This record does define the `owner` variable but with the wrong type.
    record Token {
        owner: bool,
    }

    @noupgrade
    constructor() {}
}

fn main(x: field) -> bool {
    return true;
}
