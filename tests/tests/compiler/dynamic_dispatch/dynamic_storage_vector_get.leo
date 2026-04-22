// Interface-based dynamic vector element read: Interface@(target)::v.get(i)
interface Logs {
    storage entries: [u64];
}

program test.aleo {
    fn query(target: field, i: u32) -> Final {
        return final { finalize_query(target, i); };
    }
}

final fn finalize_query(target: field, i: u32) {
    let val: u64? = Logs@(target)::entries.get(i);
}
