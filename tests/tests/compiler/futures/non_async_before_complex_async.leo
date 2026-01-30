program inner.aleo {
    mapping foo: u32 => u32;

    fn inner(a: u32) -> Final {
        return finalize(a);
    }
}

onchain fn finalize(a: u32) {
    Mapping::set(foo, 0u32, a);
}

// --- Next Program --- //

import inner.aleo;
program mid.aleo {
    fn mid(a: u32) -> Final {
        let f1: Final = inner.aleo/inner(0u32);
        let f2: Final = inner.aleo/inner(1u32);
        let f:Final = finalize(f2, f1);
        return f;
    }
    fn dummy() {}
}

onchain fn finalize(f1: Final, f2: Final) {
    f1.run();
    f2.run();
}

// --- Next Program --- //

import inner.aleo;
import mid.aleo;
program outer.aleo {
    fn outer_1(a: u32) -> Final {
        mid.aleo/dummy();
        let f1: Final = mid.aleo/mid(0u32);
        let f2: Final = mid.aleo/mid(1u32);
        let f:Final = finalize(f1, f2);
        return f;
    }

    fn outer_2(a: u32) -> Final {
        let f1: Final = mid.aleo/mid(0u32);
        mid.aleo/dummy();
        let f2: Final = mid.aleo/mid(1u32);
        let f:Final = finalize(f1, f2);
        return f;
    }
}

onchain fn finalize(f1: Final, f2: Final) {
    f1.run();
    f2.run();
}
