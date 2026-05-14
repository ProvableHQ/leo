// FAIL: views are leaves even across program boundaries. snarkVM rejects `call`
// inside a `view` block, so a view in `caller.aleo` cannot invoke a view in an
// imported program either.
program data.aleo {
    mapping balances: address => u64;

    view fn get_balance(addr: address) -> u64 {
        return balances.get_or_use(addr, 0u64);
    }

    fn touch(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //
import data.aleo;

program caller.aleo {
    view fn proxy_balance(addr: address) -> u64 {
        return data.aleo::get_balance(addr);
    }

    fn touch(x: u32) -> u32 {
        return x;
    }

    @noupgrade
    constructor() {}
}
