// Test: an interface defined in a submodule of the current program that requires a record
// prototype. The implementing program supplies its own locally-defined record — should pass.
//
// Without the module-prefix fix in flatten_interface, the prototype record location would be
// constructed as (test_aleo, [Token]) while the type Token inside the module resolves to
// (test_aleo, [interfaces, Token]). The contains check would miss and validation would fail.

program test.aleo: interfaces::HasToken {
    record Token {
        owner: address,
    }

    fn transfer(t: Token) -> Token {
        return t;
    }

    fn main() {}
}

// --- Next Module: interfaces.leo --- //

interface HasToken {
    record Token {
        owner: address,
        ..
    }
    fn transfer(t: Token) -> Token;
}
