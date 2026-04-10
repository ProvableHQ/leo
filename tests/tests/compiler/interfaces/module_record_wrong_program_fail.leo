// Test: a program that implements an interface requiring a record prototype must use its OWN
// record, not a same-named record imported from a different program.
//
// Here, `test.aleo` has no `Token` record of its own and tries to satisfy the interface
// requirement `fn transfer(t: Token) -> Token` using `other.aleo`'s `Token`. This must fail.

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

program test.aleo: lib.aleo::HasToken {
    fn transfer(t: other.aleo::Token) -> other.aleo::Token {
        return t;
    }

    fn main() {}
}
