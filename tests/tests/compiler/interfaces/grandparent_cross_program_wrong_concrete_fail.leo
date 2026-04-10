// Test: abstract record is declared in a grandparent interface (grandparent.aleo/IGP),
// inherited by a parent interface (parent.aleo/IP), and the implementing program
// (test.aleo) tries to satisfy the abstract-record requirement using a concrete
// external record from other.aleo instead of its own record.
//
// This would have been accepted by the old IndexSet<Symbol> approach because
// other.aleo::Token has the same name `Token` as the abstract prototype. The
// fix keyed abstract records by their fully-qualified Location, so only
// test.aleo/[Token] (program == self.current_program) can satisfy the requirement.

interface IGP {
    record Token {
        owner: address,
        ..
    }
    fn fn1(t: Token) -> Token;
}

program grandparent.aleo {
    fn dummy() {}
}

// --- Next Program --- //
import grandparent.aleo;

interface IP: grandparent.aleo::IGP {
    fn fn2(t: Token) -> Token;
}

program parent.aleo {
    fn dummy() {}
}

// --- Next Program --- //

program other.aleo {
    record Token {
        owner: address,
    }

    fn dummy() {}
}

// --- Next Program --- //
import grandparent.aleo;
import parent.aleo;
import other.aleo;

program test.aleo: parent.aleo::IP {
    record Token {
        owner: address,
    }

    // Uses other.aleo::Token instead of own Token — should fail for both fn1 and fn2.
    fn fn1(t: other.aleo::Token) -> other.aleo::Token {
        return t;
    }

    fn fn2(t: other.aleo::Token) -> other.aleo::Token {
        return t;
    }

    fn main() {}
}
