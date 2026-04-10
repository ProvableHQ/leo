// Test: an interface that declares an abstract record AND references a concrete external
// record with the SAME name in a different function signature.
//
// `lib.aleo::HasMixed` requires:
//   - `abstract_transfer`: must use the implementing program's own `Token` (abstract prototype).
//   - `external_transfer`: must use `other.aleo::Token` exactly (concrete external reference).
//
// Both have the record name `Token`, but they resolve to different fully-qualified locations:
// `lib.aleo/[Token]` (abstract) vs `other.aleo/[Token]` (external). The implementation here
// is correct and should compile without errors.

program other.aleo {
    record Token {
        owner: address,
    }

    fn dummy() {}
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

    // Satisfies the abstract `record Token;` requirement with test.aleo's own Token.
    fn abstract_transfer(t: Token) -> Token {
        return t;
    }

    // Satisfies the concrete external requirement with other.aleo::Token exactly.
    fn external_transfer(t: other.aleo::Token) -> other.aleo::Token {
        return t;
    }

    fn main() {}
}
