// Safe: No external calls at all — only local state operations.
program test.aleo {
    mapping balances: u32 => u32;

    fn do_work(a: u32) -> Final {
        return final { finalize_do_work(a); };
    }
}

final fn finalize_do_work(a: u32) {
    // No interactions — just local state ops
    let current: u32 = Mapping::get_or_use(balances, 0u32, 0u32);
    Mapping::set(balances, 0u32, current + a);
    let other: u32 = Mapping::get_or_use(balances, 1u32, 0u32);
    Mapping::set(balances, 1u32, other + 1u32);
}
