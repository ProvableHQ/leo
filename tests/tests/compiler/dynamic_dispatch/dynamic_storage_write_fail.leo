// Writing to an interface storage variable via dynamic dispatch must be rejected;
// dynamic access is read-only.
interface Counter {
    storage total: u64;
}

program test.aleo {
    fn update(target: field) -> Final {
        return final { finalize_update(target); };
    }
}

final fn finalize_update(target: field) {
    Counter@(target)::total = 5u64;
}
