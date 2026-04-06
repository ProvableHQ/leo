// FAIL: _dynamic_call is only allowed in entry points (transitions), not final functions.
program test.aleo {
    fn main(target: field) -> Final {
        return final { finalize_main(target); };
    }
}

final fn finalize_main(target: field) {
    let val: u64 = _dynamic_call::[u64](target, target, target);
}
