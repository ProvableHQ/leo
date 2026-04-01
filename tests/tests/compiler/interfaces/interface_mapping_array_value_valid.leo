// Test: interface mapping with an array value type compiles end-to-end.

interface TokenStorage {
    mapping balances: address => [u64; 8];
}

program test.aleo: TokenStorage {
    mapping balances: address => [u64; 8];

    fn foo(owner: address) -> Final {
        return final { fin_foo(owner); };
    }
}

final fn fin_foo(owner: address) {
    let val: [u64; 8] = balances.get_or_use(owner, [0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64]);
}
