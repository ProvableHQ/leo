// FAIL: dynamic storage reads are only allowed in a finalize context.
interface Counter {
    storage total: u64;
    storage entries: [u64];
}

program test.aleo {
    fn bare_read(target: field) -> u64 {
        let v: u64? = Counter@(target)::total;
        return v.unwrap_or(0u64);
    }

    fn vector_get(target: field, i: u32) -> u64 {
        let v: u64? = Counter@(target)::entries.get(i);
        return v.unwrap_or(0u64);
    }

    fn vector_len(target: field) -> u32 {
        return Counter@(target)::entries.len();
    }
}
