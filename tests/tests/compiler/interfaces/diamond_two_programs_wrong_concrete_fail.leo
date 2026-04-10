// Test: same diamond setup as diamond_two_programs_valid, but the implementing program
// uses lib1.aleo::Token (a concrete external reference) instead of its own Token.
//
// The old IndexSet<Symbol> approach would have accepted this: lib1.aleo::Token has
// name `Token`, which appeared in record_names, so the name-only match succeeded.
// The fix uses fully-qualified Locations: lib1.aleo/[Token] is abstract but the
// concrete type's program must equal self.current_program (test.aleo). Since
// lib1.aleo != test.aleo, both fn signatures are rejected.

interface ILeft {
    record Token {
        owner: address,
        ..
    }
    fn transfer(t: Token) -> Token;
}

program lib1.aleo {
    fn dummy() {}
}

// --- Next Program --- //

interface IRight {
    record Token {
        owner: address,
        ..
    }
    fn stake(t: Token) -> Token;
}

program lib2.aleo {
    fn dummy() {}
}

// --- Next Program --- //
import lib1.aleo;
import lib2.aleo;

program test.aleo: lib1.aleo::ILeft + lib2.aleo::IRight {
    record Token {
        owner: address,
    }

    // Uses lib1.aleo::Token instead of own Token — should fail.
    fn transfer(t: lib1.aleo::Token) -> lib1.aleo::Token {
        return t;
    }

    fn stake(t: lib1.aleo::Token) -> lib1.aleo::Token {
        return t;
    }

    fn main() {}
}
