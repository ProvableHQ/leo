// _dynamic_contains requires exactly 4 arguments.
program test.aleo {
    fn check(prog: field, net: field, m: field) -> Final {
        return final { finalize_check(prog, net, m); };
    }
}

final fn finalize_check(prog: field, net: field, m: field) {
    // Only 3 args — missing key.
    let exists: bool = _dynamic_contains(prog, net, m);
    assert(exists);
}
