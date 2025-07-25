
// The 'test_dep' program.
program test_dep.aleo {
    mapping Yo: u32 => u32;
    record yeets {
        owner: address,
        val: u32,
    }

    async transition main_dep(a:u32) -> (yeets, Future) {
        let b = 1u32;
        // Captures `a` then `b`
        let f: Future = async {
            Mapping::set(Yo, a, b);
            let c:u32 = a + b;
        };
        let l: yeets = yeets {owner: self.signer, val: 1u32};
        return (l, f);
    }
}

// --- Next Program --- //

// The 'test' program.
import test_dep.aleo;
program test.aleo {
    mapping ayo: u32 => u32;

    async transition main(a:u32)-> (u32, Future) {
        let (y,f): (test_dep.aleo/yeets, Future) = test_dep.aleo/main_dep(10u32);
        let (y2,f2): (test_dep.aleo/yeets, Future) = test_dep.aleo/main_dep(1u32);
        let val:u32 = f.0;
        let b = 1u32;
        // captures `b` then `f` then `f2`
        let f3: Future = async {
            if b == 1u32 {
                Future::await(f);
                f2.await();
            }

            if b == 2u32 {
                Mapping::set(ayo, 1u32, 1u32);
            }

            let total: u32 = f.0 + f2.0;
            Mapping::set(ayo, 1u32, total);
        };

        let total: u32 = f.0 + f2.0 + 1u32;
        return (total + val + f.0*2u32, f3);
    }
}

// --- Next Program --- //

// The 'wrapper' program.
import test.aleo;
program wrapper.aleo {
    async transition main(public a: u32, b: u32) -> (u32, Future){
        let (val, f): (u32, Future) = test.aleo/main(1u32);
        let (val2, f2): (u32, Future) = test.aleo/main(1u32);
        let (val3, f3): (u32, Future) = test.aleo/main(1u32);
        // Captures `f` then `f2` then `f3`
        let f4: Future = async {
            f.await();
            f2.await();
            f3.await();
        };
        return (val, f4);
    }
}

// --- Next Program --- //

import wrapper.aleo;
import test_dep.aleo;
program big_wrapper.aleo {
    async transition main(public a: u32, b: u32) -> (u32, Future){
        let (val, f): (u32, Future) = wrapper.aleo/main(10u32, 10u32);
        let f2:Future = finalize_main(f);
        return (f2.0.0.1.0, f2);
    }

    async function finalize_main(f: Future) {
        f.await();
    }
}
