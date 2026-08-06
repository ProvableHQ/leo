export interface CompositeKinds {
    record Token;

    fn echo_record(token: Token) -> Token;
    fn echo_struct(value: types::Token) -> types::Token;
}

program foo.aleo : CompositeKinds {
    record Token {
        owner: address,
        amount: u32,
    }

    fn echo_record(token: Token) -> Token {
        return token;
    }

    fn echo_struct(value: types::Token) -> types::Token {
        return value;
    }

    @noupgrade
    constructor() {}
}
