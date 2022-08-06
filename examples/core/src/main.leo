// This function takes as input a field `a` and calls several core circuit functions.
// Core circuit functions are built-in to the Leo language and call handwritten, optimized circuits in the AVM.
// To call a core circuit function, use the correct capitalized circuit identifier followed by two colons
// and then the function. Example: `Pedersen64::hash()`.
@program
function main(a: field) -> field {
    let b: field = BHP256::hash(a);
    let c: field = Poseidon2::hash(b);
    let d: field = BHP256::commit(c, 1scalar);
    return d;
}
