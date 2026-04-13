// Test: diamond inheritance where both parent interfaces live in different submodules
// (imod1 and imod2) of the same program, and both declare `record Token`.
//
// The record-merging path in `flatten_interface` uses a name-only match
// (`l.path.last() == Some(parent_name)`) to recognise the same abstract record across
// parents. This means (test.aleo, [imod1, Token]) and (test.aleo, [imod2, Token]) are
// treated as the same abstract record and an alias entry is created for the second
// parent's location. The implementing program supplies one own Token record, which must
// satisfy both abstract locations via the alias — should pass.

program test.aleo: imod1::ILeft + imod2::IRight {
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

// --- Next Module: imod1.leo --- //

interface ILeft {
    record Token {
        owner: address,
        ..
    }
    fn transfer(t: Token) -> Token;
}

// --- Next Module: imod2.leo --- //

interface IRight {
    record Token {
        owner: address,
        ..
    }
    fn stake(t: Token) -> Token;
}
