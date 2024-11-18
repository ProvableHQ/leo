/*
namespace = "Compile"
expectation = "Fail"
*/
program inner.aleo {
    mapping foo: u32 => u32;

    async transition inner(a: u32) -> Future {
        return finalize(a);
    }

    async function finalize(a: u32) {
        Mapping::set(foo, 0u32, a);
    }
}

// --- Next Program --- //

import inner.aleo;
program mid.aleo {
    async transition mid(a: u32) -> Future {
        let f1: Future = inner.aleo/inner(0u32);
        let f2: Future = inner.aleo/inner(1u32);
        let f:Future = finalize(f2, f1);
        return f;
    }

    async function finalize(f1: Future, f2: Future) {
        f1.await();
        f2.await();
    }

    transition dummy() {}
}

// --- Next Program --- //

import inner.aleo;
import mid.aleo;
program outer.aleo {
    async transition outer_1(a: u32) -> Future {
        mid.aleo/dummy();
        let f1: Future = mid.aleo/mid(0u32);
        let f2: Future = mid.aleo/mid(1u32);
        let f:Future = finalize(f1, f2);
        return f;
    }

    async transition outer_2(a: u32) -> Future {
        let f1: Future = mid.aleo/mid(0u32);
        mid.aleo/dummy();
        let f2: Future = mid.aleo/mid(1u32);
        let f:Future = finalize(f1, f2);
        return f;
    }

    async function finalize(f1: Future, f2: Future) {
        f1.await();
        f2.await();
    }
}
