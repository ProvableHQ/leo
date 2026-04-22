// Interface-based dynamic mapping access when the interface declares multiple mappings.
interface DeFi {
    mapping balances: address => u64;
    mapping stakes: address => u64;
}

program test.aleo {
    fn check(target: field, key: address) -> Final {
        return final { finalize_check(target, key); };
    }
}

final fn finalize_check(target: field, key: address) {
    let bal: u64 = DeFi@(target)::balances.get_or_use(key, 0u64);
    let stake: u64 = DeFi@(target)::stakes.get_or_use(key, 0u64);
    assert(bal + stake >= 0u64);
}
