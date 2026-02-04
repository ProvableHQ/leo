// As of commit 2fdad2fa89e6e47444676dbc2558c45d53550225,
// programs with an internal record received as input
// and output by themselves were not properly cloned.
program test.aleo {
    record R {
        owner: address,
    }

    fn foo(r: R) -> R {
        // This should be cloned.
        return r;
    }
}
