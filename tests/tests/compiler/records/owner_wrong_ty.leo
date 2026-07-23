program test.aleo {
    // This record does define the `owner` variable but with the wrong type.
    record Token {
        owner: bool,
    }

    @noupgrade
    constructor() {}
}

export fn main(x: field) -> bool {
    return true;
}
