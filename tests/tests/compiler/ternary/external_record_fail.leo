
program test0.aleo {
    record R {
        owner: address,
        x: bool,
    }

    fn foo() -> R {
        return R {
            owner: self.signer,
            x: true,
        };
    }
}

// --- Next Program --- //

import test0.aleo;
program test1.aleo {
    // It's impossible to have a ternary conditional over an external record type.
    // As of commit 314d56a9e3626418e8a953ecb81f595555716459 this caused
    // a compiler panic.
    fn bar(x: bool) -> test0.aleo/R {
        let r1: test0.aleo/R = test0.aleo/foo();
        let r2: test0.aleo/R = test0.aleo/foo();
        return x ? r1 : r2;
    }
}
