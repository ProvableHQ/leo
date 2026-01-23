program singleton_storage.aleo {
    storage counter: u8;
    
    storage vec: [u8];

    fn push() -> Future {
        return async {
            vec.push(42u8);
        };
    }

    fn pop() -> Future {
        return async {
            let x = vec.pop();
            assert(x != none);
        };
    }

    fn get() -> Future {
        return async {
            let x = vec.get(3u32);
            assert(x != none);
        };
    }

    fn clear() -> Future {
        return async {
            vec.clear();
        };
    }

    fn swap_remove() -> Future {
        return async {
            let x = vec.swap_remove(3u32);
            assert(x >= 0);
        };
    }

    fn set() -> Future {
        return async {
            vec.set(3u32, 42u8);
        };
    }
}
