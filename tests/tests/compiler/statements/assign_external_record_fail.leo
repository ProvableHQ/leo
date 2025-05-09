
program test0.aleo {
    record R {
        owner: address,
        x: bool,
    }

    transition foo() -> R {
        return R {
            owner: self.signer,
            x: true,
        };
    }
}

// --- Next Program --- //

import test0.aleo;
program test1.aleo {
    // It's impossible to have a ternary conditional over an external record type,
    // and an assignment will be converted to a ternary, so this won't work.
    // As of commit 314d56a9e3626418e8a953ecb81f595555716459 this caused
    // a compiler panic.
    transition bar(x: bool) -> test0.aleo/R {
        let r1: test0.aleo/R = test0.aleo/foo();
        if x {
            r1 = test0.aleo/foo();
        }
        return r1;
    }

    transition bar2(x: bool) -> test0.aleo/R {
        let tupl: (test0.aleo/R, test0.aleo/R) = (test0.aleo/foo(), test0.aleo/foo());
        if x {
            tupl.0 = test0.aleo/foo();
        }
        return tupl.0;
    }

    transition bar3(x: bool) -> test0.aleo/R {
        let r: test0.aleo/R = test0.aleo/foo();
        r.x = x;
        return r;
    }
}
