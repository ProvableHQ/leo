
struct Foo {
    // The token amount.
    token: Token,
}

program test.aleo {
    record Token {
        // The token owner.
        owner: address,
        // The token amount.
        foo: Foo,
    }
}
