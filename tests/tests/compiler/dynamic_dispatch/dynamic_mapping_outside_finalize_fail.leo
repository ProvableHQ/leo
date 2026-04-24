// Error: dynamic mapping access outside finalize context.
interface Bank {
    mapping balances: address => u64;
}

program test.aleo {
    fn query(target: field, key: address) -> u64 {
        return Bank@(target)::balances.get(key);
    }
}
