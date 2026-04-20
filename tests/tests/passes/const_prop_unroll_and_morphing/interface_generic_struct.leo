// Test: monomorphization reconstructs interface function prototypes.
// A const-generic struct instantiation in an interface function prototype must be
// monomorphized the same way it would be in a concrete function. Without calling
// reconstruct_interface in monomorphization, the prototype retains the unresolved
// generic form `Pair::[4u32]` instead of the concrete monomorphized struct.

struct Pair::[N: u32] {
    first: u32,
    second: u32,
}

interface Combiner {
    fn combine(a: Pair::[4u32], b: Pair::[4u32]) -> Pair::[4u32];
}

program test.aleo: Combiner {
    fn combine(a: Pair::[4u32], b: Pair::[4u32]) -> Pair::[4u32] {
        return Pair::[4u32] { first: a.first + b.first, second: a.second + b.second };
    }
}
