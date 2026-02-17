
// The 'test_dep' program.
program test_dep.aleo {
    mapping Yo: u32 => u32;
    record yeets {
        owner: address,
        val: u32,
    }

    fn main_dep(a:u32) -> (yeets, Final) {
        let b = 1u32;
        // Captures `a` then `b`
        let f: Final = final {
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

    fn main(a:u32)-> (u32, Final) {
        let (y,f): (test_dep.aleo/yeets, Final) = test_dep.aleo/main_dep(10u32);
        let (y2,f2): (test_dep.aleo/yeets, Final) = test_dep.aleo/main_dep(1u32);
        let b = 1u32;
        // captures `b` then `f` then `f2`
        let f3: Final = final {
            if b == 1u32 {
                Final::run(f);
                f2.run();
            }

            if b == 2u32 {
                Mapping::set(ayo, 1u32, 1u32);
            }

            Mapping::set(ayo, 1u32, a);
        };

        return (a, f3);
    }
}

// --- Next Program --- //

// The 'wrapper' program.
import test.aleo;
program wrapper.aleo {
    fn main(public a: u32, b: u32) -> (u32, Final){
        let (val, f): (u32, Final) = test.aleo/main(1u32);
        let (val2, f2): (u32, Final) = test.aleo/main(1u32);
        let (val3, f3): (u32, Final) = test.aleo/main(1u32);
        // Captures `f` then `f2` then `f3`
        let f4: Final = final {
            f.run();
            f2.run();
            f3.run();
        };
        return (val, f4);
    }
}

// --- Next Program --- //

import wrapper.aleo;
import test_dep.aleo;
program big_wrapper.aleo {
    fn main(public a: u32, b: u32) -> (u32, Final){
        let (val, f): (u32, Final) = wrapper.aleo/main(10u32, 10u32);
        return (val, final { finalize_main(f); });
    }
}

final fn finalize_main(f: Final) {
    f.run();
}
