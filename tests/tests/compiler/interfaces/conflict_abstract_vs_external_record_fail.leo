// Test: two parent interfaces declare the same function name `process`, but one uses
// an abstract record prototype (Token) and the other uses a concrete external record
// (other.aleo::Token). These resolve to different fully-qualified Locations, so the
// signatures are incompatible and ICombined should fail to flatten.
//
// The old IndexSet<Symbol> approach matched both as `Token` in record_names and
// declared the signatures compatible (a false positive in prototypes_match).
// The fix checks abstract_record_locations: other.aleo/[Token] is NOT abstract
// (it is a concrete external reference), so proto_type_eq falls back to eq_user,
// which correctly identifies the mismatch.

program other.aleo {
    record Token {
        owner: address,
    }

    fn dummy() {}
}

// --- Next Program --- //
import other.aleo;

interface IAbstract {
    record Token {
        owner: address,
        ..
    }
    fn process(t: Token) -> Token;
}

interface IConcrete {
    fn process(t: other.aleo::Token) -> other.aleo::Token;
}

// ICombined inherits conflicting `process` signatures — should fail.
interface ICombined: IAbstract + IConcrete {}

program lib.aleo {
    fn dummy() {}
}

// --- Next Program --- //
import other.aleo;
import lib.aleo;

program test.aleo: lib.aleo::ICombined {
    fn main() {}
}
