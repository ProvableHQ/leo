program singleton_storage.aleo {
    storage counter: u8;
    
    storage vec: [u8];

    async transition push() -> Future {
        return async {
            vec.push(42u8);
        };
    }

    async transition pop() -> Future {
        return async {
            let x = vec.pop();
            assert(x != none);
        };
    }

    async transition get() -> Future {
        return async {
            let x = vec.get(3u32);
            assert(x != none);
        };
    }

    async transition clear() -> Future {
        return async {
            vec.clear();
        };
    }

    async transition swap_remove() -> Future {
        return async {
            let x = vec.swap_remove(3u32);
            assert(x >= 0);
        };
    }

    async transition set() -> Future {
        return async {
            vec.set(3u32, 42u8);
        };
    }
}
