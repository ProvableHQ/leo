// Test: const propagation reconstructs interfaces in program scopes, including
// array lengths that reference const variables. Without the fix to
// reconstruct_program_scope, the interface mapping key would retain the unfolded
// const variable instead of the concrete value.

const N: u32 = 2u32;

interface TokenStorage {
    mapping allowances: [address; N] => u128;
}

program test.aleo: TokenStorage {
    mapping allowances: [address; N] => u128;

    fn foo() {}
}
