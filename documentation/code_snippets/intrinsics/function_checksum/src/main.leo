// ANCHOR: file
program function_checksum_demo.aleo {
    mapping balances: address => u64;

    // A view function: a valid `function_checksum` target.
    view fn balance_of(owner: address) -> u64 {
        return balances.get_or_use(owner, 0u64);
    }

    // An entry function: also a valid `function_checksum` target.
    fn audit() -> Final {
        return final {
            // Checksum of an entry function in the current program.
            let f: [u8; 32] = Program::function_checksum(function_checksum_demo.aleo, 'audit');
            // Checksum of a view function in the current program.
            let v: [u8; 32] = Program::function_checksum(function_checksum_demo.aleo, 'balance_of');
            assert_neq(f, v);
        };
    }

    @noupgrade
    constructor() {}
}
// ANCHOR_END: file
