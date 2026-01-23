program inner.aleo {
    mapping foo: u32 => u32;

    fn inner(a: u32) -> Final {
        return async { Mapping::set(foo, 0u32, a);};
    }
}

// --- Next Program --- //

import inner.aleo;
program mid.aleo {
    fn mid(a: u32) -> Final {
        let f1: Final = inner.aleo/inner(0u32);
        let f2: Final = inner.aleo/inner(1u32);
        let f:Final = async { f2.await(); f1.await(); };
        return f;
    }

    fn dummy() {}
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
        let f:Final = async { f1.await(); f2.await(); };
        return f;
    }
}
