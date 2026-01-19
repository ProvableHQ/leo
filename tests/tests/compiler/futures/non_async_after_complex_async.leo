program inner.aleo {
    mapping foo: u32 => u32;

    async fn inner(a: u32) -> Future {
        return finalize(a);
    }
}

async fn finalize(a: u32) {
    Mapping::set(foo, 0u32, a);
}

// --- Next Program --- //

import inner.aleo;
program mid.aleo {
    async fn mid(a: u32) -> Future {
        let f1: Future = inner.aleo/inner(0u32);
        let f2: Future = inner.aleo/inner(1u32);
        let f:Future = finalize(f2, f1);
        return f;
    }
    fn dummy() {}
}

async fn finalize(f1: Future, f2: Future) {
    f1.await();
    f2.await();
}

// --- Next Program --- //

import inner.aleo;
import mid.aleo;
program outer.aleo {
    async fn outer(a: u32) -> Future {
        let f1: Future = mid.aleo/mid(0u32);
        let f2: Future = mid.aleo/mid(1u32);
        // Allow this call to remain for now.
        mid.aleo/dummy();
        let f:Future = finalize(f1, f2);
        return f;
    }
}

async fn finalize(f1: Future, f2: Future) {
    f1.await();
    f2.await();
}
