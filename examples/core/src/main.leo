// The 'core' main function.
function main(a: field) -> field {
    let b: field = BHP256::hash(a);
    let c: field = Poseidon2::hash(b);
    let d: field = BHP256::commit(c, 1scalar);
    return d;
}
