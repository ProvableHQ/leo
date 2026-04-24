// Pins the behavior that dynamic mapping access is ALLOWED inside a conditional
// branch in a finalize context. This is intentional and matches the `_dynamic_get`,
// `_dynamic_contains`, and `_dynamic_get_or_use` intrinsics. It differs from dynamic
// function calls (`Interface@(t)::func(args)`), which `validate_dynamic_call_scope`
// rejects inside conditionals.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn maybe_query(target: field, key: address, cond: bool) -> Final {
        return final { finalize_maybe_query(target, key, cond); };
    }
}

final fn finalize_maybe_query(target: field, key: address, cond: bool) {
    if cond {
        let a: u64 = Bank@(target)::balances.get(key);
        let b: bool = Bank@(target)::balances.contains(key);
        let c: u64 = Bank@(target)::balances.get_or_use(key, 0u64);
    }
}
