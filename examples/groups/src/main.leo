program groups.aleo {
    // This function takes a group coordinate as input `a` and performs several operations which should output the `0group`.
    // Note that the operations can be called as associated functions on the `a` variable.

    transition main(a: group) -> group {
        // unary
        let e: group = a.double(); // 2a
        let g: group = e.neg();    // -2a

        // binary
        let j: group = (a * 2scalar).add(g);

        return j;
    }
}

