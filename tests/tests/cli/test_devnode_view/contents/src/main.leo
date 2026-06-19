// Exercises the `leo devnode` view evaluation endpoints:
//   POST /{network}/program/{id}/view/{function}
//   POST /{network}/program/{id}/view/{function}/{height}
// `get_balance` reads from an empty mapping, so it deterministically returns the default.
program test_devnode_view.aleo {
    mapping balances: address => u64;

    view fn get_balance(addr: address) -> u64 {
        return balances.get_or_use(addr, 0u64);
    }

    // An entry point is required for the program to be valid and deployable.
    fn noop(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}
