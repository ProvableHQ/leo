// Test: IChild (in interfaces.leo) inherits f1 from IBase (also in interfaces.leo).
// The implementing program defines its own Token but uses other.aleo::Token in f1
// — should fail because the prototype record Token must be satisfied by the
// implementing program's own record.

program other.aleo {
    record Token {
        owner: address,
    }

    fn dummy() {}
}

// --- Next Program --- //
import other.aleo;

program test.aleo: interfaces::IChild {
    record Token {
        owner: address,
    }

    fn f1(t: other.aleo::Token) -> other.aleo::Token {
        return t;
    }

    fn f2(t: Token) -> Token {
        return t;
    }

    fn main() {}
}

// --- Next Module: interfaces.leo --- //

interface IBase {
    record Token {
        owner: address,
        ..
    }
    fn f1(t: Token) -> Token;
}

interface IChild: IBase {
    record Token {
        owner: address,
        ..
    }
    fn f2(t: Token) -> Token;
}
