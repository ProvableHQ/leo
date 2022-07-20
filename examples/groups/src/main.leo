// The 'groups' main function.
function main(a: group) -> group {
    // unary
    let e: group = a.double(); // 2a
    let g: group = e.neg();    // -2a
    
    // binary
    let j: group = (a * 2scalar).add(g);

    return j;
}
