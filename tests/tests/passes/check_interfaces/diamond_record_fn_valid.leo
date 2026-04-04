// Test: diamond interface inheritance with record types in function signatures,
// where the interfaces live in a library module.
//
// The interfaces are in ifaces::interfaces, so their resolved record paths differ
// from the implementing program's paths (e.g. ifaces::interfaces/[Token] vs
// test.aleo/[Token]). type_eq_with_record_names must match them by name.
//
// Exercises three paths in type_eq_with_record_names:
//
//   1. Composite branch via prototypes_match (inside flatten_interface):
//      IDiamond inherits issue() from both ILeft and IRight (both via IBase).
//      flatten_interface calls prototypes_match with inheritance_record_names = {Token}
//      to verify the two copies are compatible — Token's return type is compared by
//      name rather than resolved path.
//
//   2. Tuple branch via function_matches_prototype (check_program_implements_interface):
//      split() returns (Token, Token). type_eq_with_record_names recurses into the
//      tuple elements and matches each Token by name.
//
//   3. Composite branch via function_matches_prototype:
//      issue(), split(), and owner_of() all have Token as a parameter or return type.

// --- library: ifaces --- //

// --- Next Module: interfaces.leo --- //

interface IBase {
    record Token {
        owner: address,
        ..
    }
    fn issue(owner: address) -> Token;
}

interface ILeft: IBase {
    fn split(t: Token) -> (Token, Token);
}

interface IRight: IBase {
    fn owner_of(t: Token) -> address;
}

// IDiamond inherits both ILeft and IRight (diamond pattern).
// flatten_interface encounters issue() in both parents and calls prototypes_match
// with inheritance_record_names = {Token} to verify they are compatible.
interface IDiamond: ILeft + IRight {}

// --- Next Program --- //

program test.aleo: ifaces::interfaces::IDiamond {
    record Token {
        owner: address,
        amount: u64,
    }

    fn issue(owner: address) -> Token {
        return Token { owner: owner, amount: 0u64 };
    }

    fn split(t: Token) -> (Token, Token) {
        let half: u64 = t.amount / 2u64;
        let a: Token = Token { owner: t.owner, amount: half };
        let b: Token = Token { owner: t.owner, amount: half };
        return (a, b);
    }

    fn owner_of(t: Token) -> address {
        return t.owner;
    }

    @noupgrade
    constructor() {}
}
