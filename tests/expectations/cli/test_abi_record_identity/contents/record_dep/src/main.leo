program record_dep.aleo {
    record Token {
        owner: address,
        amount: u32,
    }

    fn echo_token(token: Token) -> Token {
        return token;
    }

    @noupgrade
    constructor() {}
}
