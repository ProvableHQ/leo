// Test: interface in a submodule requires a record prototype Token; the implementing program
// has its own Token but the function uses other.aleo::Token instead — should fail.

program other.aleo {
    record Token {
        owner: address,
    }

    fn dummy() {}

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import other.aleo;

program test.aleo: interfaces::HasToken {
    record Token {
        owner: address,
    }

    fn transfer(t: other.aleo::Token) -> other.aleo::Token {
        return t;
    }

    fn main() {}

    @noupgrade
    constructor() {}
}

// --- Next Module: interfaces.leo --- //

export interface HasToken {
    record Token {
        owner: address,
        ..
    }
    fn transfer(t: Token) -> Token;
}
