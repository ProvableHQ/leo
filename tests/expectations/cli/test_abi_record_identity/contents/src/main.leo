import interface_dep.aleo;
import record_dep.aleo;

export interface CompositeKinds {
    record Token;

    fn echo_record(token: Token) -> Token;
    fn echo_struct(value: types::Token) -> types::Token;
}

export interface PrototypeOnly {
    record PrototypeToken;

    fn echo_prototype(token: PrototypeToken) -> PrototypeToken;
}

program foo.aleo : CompositeKinds + interface_dep.aleo::ExternalRecords {
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

    fn echo_external(token: record_dep.aleo::Token) -> record_dep.aleo::Token {
        return token;
    }

    @noupgrade
    constructor() {}
}
