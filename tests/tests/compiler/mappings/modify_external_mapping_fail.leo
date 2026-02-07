program provider.aleo {
    mapping balances: u32 => u32;

    async transition init() -> Future {
        return async {
            balances.set(1u32, 100u32);
            balances.set(2u32, 200u32);
        };
    }

    @noupgrade
    async constructor() {}
}

// --- Next Program --- //

import provider.aleo;

program user.aleo {
    async transition try_modify() -> Future {
        return async {
            //
            // All of these must be rejected — external mapping mutation is illegal.
            //
            provider.aleo/balances.set(3u32, 300u32);  // ❌ illegal external mutation
            provider.aleo/balances.remove(1u32);       // ❌ illegal external mutation
        };
    }

    @noupgrade
    async constructor() {}
}

