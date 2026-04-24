// FAIL: vector `.len()` takes zero arguments.
interface Logs {
    storage entries: [u64];
}

program test.aleo {
    fn query(target: field) -> Final {
        return final { finalize_query(target); };
    }
}

final fn finalize_query(target: field) {
    let n: u32 = Logs@(target)::entries.len(0u32);
}
