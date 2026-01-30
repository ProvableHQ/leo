program inner.aleo {
    mapping foo: u32 => u32;

    fn inner(a: u32) -> Final {
        return final { Mapping::set(foo, 0u32, a);};
    }
}

// --- Next Program --- //

import inner.aleo;
program mid.aleo {
    fn mid(a: u32) -> Final {
        let f1: Final = inner.aleo/inner(0u32);
        let f2: Final = inner.aleo/inner(1u32);
        let f:Final = final { f2.run(); f1.run(); };
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
        let f:Final = final { f1.run(); f2.run(); };
        return f;
    }
}
