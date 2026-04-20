// Test: module interface IChild (in interfaces.leo) inherits record prototype Token and
// function f1 from module interface IBase (also in interfaces.leo). Both interfaces are
// in the same submodule so IChild: IBase resolves correctly. The implementing program
// provides its own Token record and both functions — should pass.
//
// `build_inheritance_graph` seeds the graph from `input.parents`, so the module-level
// interface IChild (and its ancestor IBase) are included. `transitive_closure(IChild)`
// returns `{IBase}`, enabling `flatten_interface` to merge f1 and IBase's Token alias.
// `prototype_record_locations` contains `(test.aleo, [interfaces, Token])`.
// Both f1 and f2 validate correctly against the implementing program's own Token record.
//
// Note: cross-sibling-module interface inheritance (e.g. `interfaces.leo` inheriting from
// `base.leo`) is not yet supported due to path resolution limitations.

program test.aleo: interfaces::IChild {
    record Token {
        owner: address,
    }

    fn f1(t: Token) -> Token {
        return t;
    }

    fn f2(t: Token) -> Token {
        return t;
    }

    fn main() {}
}

// --- Next Module: interfaces.leo --- //

interface IBase {
    record Token {
        owner: address,
        ..
    }
    fn f1(t: Token) -> Token;
}

interface IChild: IBase {
    record Token {
        owner: address,
        ..
    }
    fn f2(t: Token) -> Token;
}
