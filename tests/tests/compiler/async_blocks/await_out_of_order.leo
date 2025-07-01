program test.aleo {
    mapping foo: u32 => u32;
    async transition main_inner(public a: u32, public b: u32) -> Future {
        return async {
            Mapping::set(foo, a, b);
        };
    }
}

// --- Next Program --- //

import test.aleo;
program basic.aleo {
    async transition main(public a: u32, b: u32) -> Future {
        let f1: Future = test.aleo/main_inner(0u32, 0u32);
        let f2: Future = test.aleo/main_inner(1u32, 1u32);

        // No "out of order" warning since the futures are captured in the order they appear in inside the block anyways.
        let f:Future = async {
            f2.await();
            f1.await();
        };
        return f;
    }
}
