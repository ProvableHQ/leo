
program test0.aleo {
    record R {
        owner: address,
    }

    transition foo() -> R {
        return R {
            owner: self.signer,
        };
    }
}

// --- Next Program --- //

import test0.aleo;
program test1.aleo {
    // This should compile fine without attempting to clone.
    transition bar(x: test0.aleo/R) -> test0.aleo/R {
        return x;
    }
}
