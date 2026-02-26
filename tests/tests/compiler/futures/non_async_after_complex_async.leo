program inner.aleo {
    mapping foo: u32 => u32;

    fn inner(a: u32) -> Final {
        return final { finalize(a); };
    }
}

final fn finalize(a: u32) {
    Mapping::set(foo, 0u32, a);
}

// --- Next Program --- //

import inner.aleo;
program mid.aleo {
    fn mid(a: u32) -> Final {
        let f1: Final = inner.aleo/inner(0u32);
        let f2: Final = inner.aleo/inner(1u32);
        let f:Final = final { finalize(f2, f1); };
        return f;
    }
    fn dummy() {}
}

final fn finalize(f1: Final, f2: Final) {
    f1.run();
    f2.run();
}

// --- Next Program --- //

import inner.aleo;
import mid.aleo;
program outer.aleo {
    fn outer(a: u32) -> Final {
        let f1: Final = mid.aleo/mid(0u32);
        let f2: Final = mid.aleo/mid(1u32);
        // Allow this call to remain for now.
        mid.aleo/dummy();
        let f:Final = final { finalize(f1, f2); };
        return f;
    }
}

final fn finalize(f1: Final, f2: Final) {
    f1.run();
    f2.run();
}
