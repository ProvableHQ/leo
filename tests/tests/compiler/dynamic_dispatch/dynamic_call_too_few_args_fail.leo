// FAIL: _dynamic_call requires at least 3 arguments (program, network, function).
program test.aleo {
    fn main(target: field) -> u64 {
        return _dynamic_call::[u64](target);
    }
}
