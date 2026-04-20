// Test: diamond inheritance across two different library programs (lib1.aleo, lib2.aleo),
// both defining an abstract `record Token` with the same name but different fully-qualified
// locations (lib1.aleo/[Token] and lib2.aleo/[Token]).
//
// The implementing program provides ONE own Token record, which must satisfy BOTH
// abstract requirements. Our fix stores both abstract locations and checks that the
// concrete type belongs to the current program — the alias entry for the second
// parent's location makes this work correctly.

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

    fn transfer(t: Token) -> Token {
        return t;
    }

    fn stake(t: Token) -> Token {
        return t;
    }

    fn main() {}
}
