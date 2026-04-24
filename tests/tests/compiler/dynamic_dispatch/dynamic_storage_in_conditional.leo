// Pins the behavior that dynamic storage access is ALLOWED inside a conditional
// branch in a finalize context. This covers both singleton bare reads, vector `.get(i)`,
// and vector `.len()`. It is analogous to `dynamic_mapping_in_conditional.leo`.
interface Counter {
    storage total: u64;
    storage entries: [u64];
}

program test.aleo {
    fn maybe_query(target: field, i: u32, cond: bool) -> Final {
        return final { finalize_maybe_query(target, i, cond); };
    }
}

final fn finalize_maybe_query(target: field, i: u32, cond: bool) {
    if cond {
        let a: u64? = Counter@(target)::total;
        let b: u64? = Counter@(target)::entries.get(i);
        let c: u32 = Counter@(target)::entries.len();
    }
}
