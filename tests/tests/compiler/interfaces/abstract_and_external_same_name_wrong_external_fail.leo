// Test: an interface with both an abstract `record Token;` and a concrete external
// `other.aleo::Token` reference in different function signatures (same record name).
//
// The implementing program correctly satisfies `abstract_transfer` with its own Token,
// but incorrectly satisfies `external_transfer` with its own Token instead of
// `other.aleo::Token`. This must be rejected: `other.aleo::Token` is a concrete external
// reference that must match exactly, not an abstract prototype to be matched by name.

program other.aleo {
    record Token {
        owner: address,
    }
}

// --- Next Program --- //
import other.aleo;

interface HasMixed {
    record Token {
        owner: address,
        ..
    }
    fn abstract_transfer(t: Token) -> Token;
    fn external_transfer(t: other.aleo::Token) -> other.aleo::Token;
}

program lib.aleo {
    fn dummy() {}
}

// --- Next Program --- //
import lib.aleo;
import other.aleo;

program test.aleo: lib.aleo::HasMixed {
    record Token {
        owner: address,
    }

    fn abstract_transfer(t: Token) -> Token {
        return t;
    }

    // Wrong: interface requires other.aleo::Token, not test.aleo::Token.
    fn external_transfer(t: Token) -> Token {
        return t;
    }

    fn main() {}
}
