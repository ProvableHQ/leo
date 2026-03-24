// FAIL: _dynamic_call cannot be used in a finalize (final fn) context.
program test.aleo {
    mapping counts: u32 => u32;

    fn main(target: field) -> Final {
        return final { finalize_main(target); };
    }
}

final fn finalize_main(target: field) {
    let val: u64 = _dynamic_call::[u64](target, target, target);
    Mapping::set(counts, 0u32, 0u32);
}
