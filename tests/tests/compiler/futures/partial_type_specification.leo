
// The 'test_dep' program.

program test_dep.aleo {
    record yeets {
        owner: address,
        val: u32,
    }

    mapping Yo: u32 => u32;
    fn main_dep(a:u32) -> (yeets, Final<Fn(u32)>) {
        let f: Final<Fn(u32)> = final { finalize_main_dep(a, 1u32); };
        let l: yeets = yeets {owner: self.signer, val: 1u32};
        return (l, f);
    }
    fn main_dep_2(a:u32) -> (yeets, Final<Fn()>) {
        let f: Final<Fn()> = final { finalize_main_dep_2(); };
        let l: yeets = yeets {owner: self.signer, val: 1u32};
        return (l, f);
    }
}

final fn finalize_main_dep(a:u32, b:u32) {
    Mapping::set(Yo, a, b);
    let c:u32 = a + b;
}

final fn finalize_main_dep_2() {
    Mapping::set(Yo, 1u32, 1u32);
}

// --- Next Program --- //

// The 'test' program.
import test_dep.aleo;
program test.aleo {
    mapping ayo: u32 => u32;

    fn main(a:u32)-> (u32, Final) {
        let (y,f): (test_dep.aleo/yeets, Final) = test_dep.aleo/main_dep(10u32);
        let (y2,f2): (test_dep.aleo/yeets, Final) = test_dep.aleo/main_dep(1u32);
        let f3: Final = final { finalize_main(f, f2, 1u32); };
        return (a, f3);
    }
}

final fn finalize_main(f: Final, f2: Final, a: u32) {
    // f.run();
    if a == 1u32 {
        Final::run(f);
        f2.run();
    }

    if a == 2u32 {
        //f2.run();
        Mapping::set(ayo, 1u32, 1u32);
    }

    Mapping::set(ayo, 1u32, a);
}

// --- Next Program --- //

// The 'wrapper' program.
import test.aleo;
program wrapper.aleo {
    fn main(public a: u32, b: u32) -> (u32, Final){
        let (val, f): (u32, Final) = test.aleo/main(1u32);
        let (val2, f2): (u32, Final) = test.aleo/main(1u32);
        let (val3, f3): (u32, Final) = test.aleo/main(1u32);
        let f4: Final = final { finalize_main(f,f2,f3); };
        return (val, f4);
    }
}

final fn finalize_main(f: Final, f2: Final, f3: Final) {
    f.run();
    f2.run();
    f3.run();
}

// --- Next Program --- //

import wrapper.aleo;
import test_dep.aleo;
program big_wrapper.aleo {
    fn main(public a: u32, b: u32) -> (u32, Final<Fn(Final<Fn(Final<Fn(Final<Fn(u32)>,Final<Fn(u32)>)>,Final<Fn(Final<Fn(u32)>,Final<Fn(u32)>)>,Final<Fn(Final<Fn(u32)>,Final<Fn(u32)>)>)>)>){
        let (val, f): (u32, Final) = wrapper.aleo/main(10u32, 10u32);
        return (b, final { f.run(); });
    }
}
