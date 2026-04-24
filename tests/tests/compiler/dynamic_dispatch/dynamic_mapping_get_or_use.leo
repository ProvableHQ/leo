// Interface-based dynamic mapping get_or_use: Interface@(target)::mapping_name.get_or_use(key, default)
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn balance_or_zero(target: field, key: address) -> Final {
        return final { finalize_balance_or_zero(target, key); };
    }
}

final fn finalize_balance_or_zero(target: field, key: address) {
    let val: u64 = Bank@(target)::balances.get_or_use(key, 0u64);
}
