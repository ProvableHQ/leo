program provider.aleo {
    mapping balances: u32 => u32;

    fn init() -> Final {
        return final {
            balances.set(1u32, 100u32);
            balances.set(2u32, 200u32);
        };
    }

    @noupgrade
    constructor() {}
}

// --- Next Program --- //

import provider.aleo;

program user.aleo {
    fn try_modify() -> Final {
        return final {
            //
            // All of these must be rejected — external mapping mutation is illegal.
            //
            provider.aleo/balances.set(3u32, 300u32);  // ❌ illegal external mutation
            provider.aleo/balances.remove(1u32);       // ❌ illegal external mutation
        };
    }

    @noupgrade
    constructor() {}
}

