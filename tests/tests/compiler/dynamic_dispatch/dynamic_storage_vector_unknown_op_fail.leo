// FAIL: unknown vector operation; only get and len are valid on external storage vectors.
interface Logs {
    storage entries: [u64];
}

program test.aleo {
    fn query(target: field, v: u64) -> Final {
        return final { finalize_query(target, v); };
    }
}

final fn finalize_query(target: field, v: u64) {
    Logs@(target)::entries.push(v);
}
