/*
namespace: Compile
expectation: Pass
*/

// The 'test_dep' program.
program test_dep.aleo {
    mapping Yo: u32 => u32;
    record yeets {
        owner: address,
        val: u32,
    }

    async transition main_dep(a:u32) -> (yeets, Future) {
        let f: Future = finalize_main_dep(a, 1u32);
        let l: yeets = yeets {owner: self.caller, val: 1u32};
        return (l, f);
    }

    async function finalize_main_dep(a:u32, b:u32) {
        Mapping::set(Yo, a, b);
        let c:u32 = a + b;
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
        let f3: Future = finalize_main(f, f2, 1u32);
        let total: u32 = f.0 + f2.0 + 1u32;
        return (total + val + f.0*2u32, f3);
    }

    async function finalize_main(f: Future, f2: Future, a: u32) {
        // f.await();
        if a == 1u32 {
            Future::await(f);
            f2.await();
        }

        if a == 2u32 {
            //f2.await();
            Mapping::set(ayo, 1u32, 1u32);
        }

        let total: u32 = f.0 + f2.0;
        Mapping::set(ayo, 1u32, total);
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
        let f4: Future = finalize_main(f,f2,f3);
        return (val, f4);
    }

    async function finalize_main(f: Future, f2: Future, f3: Future) {
        f.await();
        f2.await();
        f3.await();
    }
}

// --- Next Program --- //

import wrapper.aleo;
import test_dep.aleo;
program big_wrapper.aleo {
    async transition main(public a: u32, b: u32) -> (u32, Future){
        let (val, f): (u32, Future) = wrapper.aleo/main(10u32, 10u32);
        let f2:Future = finalize_main(f);
        return (f2.0.0.0.0, f2);
    }

    async function finalize_main(f: Future) {
        f.await();
    }
}