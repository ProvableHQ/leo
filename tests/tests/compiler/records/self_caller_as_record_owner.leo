program test0.aleo {
    record Token {
        owner: address,
    }

    fn foo() -> Token {
        return Token { owner: std::ctx::caller() };
    }

    fn bar() -> Token {
        let t: Token = Token { owner: std::ctx::caller() };
        return t;
    }

    @noupgrade
    constructor() {}
}
