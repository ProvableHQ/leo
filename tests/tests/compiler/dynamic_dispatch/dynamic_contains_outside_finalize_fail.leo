// FAIL: _dynamic_contains can only be used in finalize context.
program test.aleo {
    fn main(prog: field, net: field, m: field, key: address) -> bool {
        return _dynamic_contains(prog, net, m, key);
    }
}
