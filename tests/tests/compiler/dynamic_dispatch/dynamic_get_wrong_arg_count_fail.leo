// FAIL: _dynamic_get requires exactly 4 arguments.
program test.aleo {
    mapping result: address => u64;

    fn read(prog: field, net: field, m: field) -> Final {
        return final { finalize_read(prog, net, m); };
    }
}

final fn finalize_read(prog: field, net: field, m: field) {
    let val: u64 = _dynamic_get::[u64](prog, net, m);
    Mapping::set(result, self.caller, val);
}
