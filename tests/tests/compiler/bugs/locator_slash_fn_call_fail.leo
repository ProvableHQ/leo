// Regression: old `/` separator in external function-call position must not panic.
// Grammar treats `credits.aleo` as a PROGRAM_REF_EXPR (address literal) and
// `/transfer(1field)` as a division expression — no parse error.
program test.aleo {
    fn f() -> field {
        return credits.aleo / transfer(1field);
    }

    @noupgrade
    constructor() {}
}
