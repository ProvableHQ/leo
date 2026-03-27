// Regression: old `/` separator in return-type position must not panic.
// Grammar produces a single-IDENT TYPE_LOCATOR for `credits.aleo`, then
// the leftover `/credits` triggers EPAR0370047 ("expected {").
program test.aleo {
    fn f() -> credits.aleo/credits {
        return 0field;
    }

    @noupgrade
    constructor() {}
}
