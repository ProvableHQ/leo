program provider.aleo {
    storage nums: [u32];

    async transition init() -> Future {
        return async {
            nums.push(1u32);
            nums.push(2u32);
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
            // Every line below should be DISALLOWED on external storage.
            //
            provider.aleo/nums.push(3u32);         // ❌ illegal external mutation
            provider.aleo/nums.pop();              // ❌ illegal external mutation
            provider.aleo/nums.set(0u32, 99u32);   // ❌ illegal external mutation
            provider.aleo/nums.clear();            // ❌ illegal external mutation
            provider.aleo/nums.swap_remove(1u32);  // ❌ illegal external mutation
        };
    }

    @noupgrade
    async constructor() {}
}
