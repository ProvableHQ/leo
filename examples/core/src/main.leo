program core.aleo {
    // This function takes as input a field `a` and calls several core functions.
    // Core functions are built-in to the Leo language and call handwritten, optimized circuits in the AVM.
    // To call a core function, use the correct capitalized identifier followed by two colons
    // and then the function name. Example: `Pedersen64::hash_to_field()`.
    transition main(a: field) -> field {
        let b: field = BHP256::hash_to_field(a);
        let c: field = Poseidon2::hash_to_field(b);
        let d: field = BHP256::commit_to_field(c, 1scalar);
        return d;
    }
}

