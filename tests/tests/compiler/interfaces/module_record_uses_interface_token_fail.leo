// Test: implementing program defines its own `Token` record but the function uses
// `lib.aleo::Token` (the interface's abstract record prototype location) instead of
// its own `bad_records.aleo::Token`. This must fail — the interface requires the
// implementing program's own Token.
//
// Note: the error output shows identical "Expected" and "Found" signatures because both
// display as `lib.aleo::Token`. The prototype record IS at `lib.aleo::Token` (the
// interface-owning program), and the function argument explicitly uses `lib.aleo::Token`
// too. The mismatch is semantic: `concrete_type_matches_proto` requires the argument to
// belong to the implementing program (`bad_records.aleo`), not to `lib.aleo`.

interface HasToken {
    record Token {
        owner: address,
        ..
    }
    fn transfer(t: Token) -> Token;
}

program lib.aleo {
    fn dummy() {}
}

// --- Next Program --- //
import lib.aleo;

program other.aleo {
    record Token {
        owner: address,
    }
}

// --- Next Program --- //
import lib.aleo;
import other.aleo;

program bad_records.aleo: lib.aleo::HasToken {
    record Token {
        owner: address,
    }

    fn transfer(t: lib.aleo::Token) -> lib.aleo::Token {
        return t;
    }

    fn main() {}
}
