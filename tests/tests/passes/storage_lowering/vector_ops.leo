program singleton_storage.aleo {
    @noupgrade
    constructor() {}

    storage counter: u8;

    storage vec: [u8];
    storage nums: [u32];

    fn push() -> Final {
        return final { vec.push(42u8); };
    }

    fn pop() -> Final {
        return final {
            let x = vec.pop();
            assert(x != none);
        };
    }

    fn get() -> Final {
        return final {
            let x = vec.get(3u32);
            assert(x != none);
        };
    }

    fn clear() -> Final {
        return final { vec.clear(); };
    }

    fn swap_remove() -> Final {
        return final {
            let x = vec.swap_remove(3u32);
            assert(x >= 0);
        };
    }

    fn set() -> Final {
        return final { vec.set(3u32, 42u8); };
    }

    fn get_statement_fallible_index(public x: u32) -> Final {
        return final { nums.get(1u32 / x); };
    }
}
