program dependent.aleo {
    async fn t1() -> Future {
        let f: Future = finalize1(1u8);
        return f;
    }
    async fn t2() -> Future {
        let f: Future = finalize2(1u8);
        return f;
    }
}

async fn finalize1(x: u8) {
    assert_eq(1u8, 1u8);
}
async fn finalize2(x: u8) {
    assert_eq(1u8, 1u8);
}

// --- Next Program --- //

import dependent.aleo;

program test.aleo {
    async fn t1() -> Future {
        let f: Future = dependent.aleo/t1();
        return finalize(f);
    }

    async fn t2() -> Future {
        let f: Future = dependent.aleo/t2();
        return finalize(f);
    }
}

async fn finalize(f: Future) {
    let x: u8 = f.0;
    f.await();
}

