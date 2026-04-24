// Interface-based dynamic singleton storage read: Interface@(target)::x
interface Counter {
    storage total: u64;
}

program test.aleo {
    fn query(target: field) -> Final {
        return final { finalize_query(target); };
    }
}

final fn finalize_query(target: field) {
    let val: u64? = Counter@(target)::total;
}
