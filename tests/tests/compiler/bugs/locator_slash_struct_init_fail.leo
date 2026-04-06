// Regression: old `/` separator in external struct-init position must not panic.
// Grammar treats `credits.aleo` as a PROGRAM_REF_EXPR (address literal) and
// `/Token { ... }` as a division expression — no parse error.
program test.aleo {
    struct Token {
        addr: address,
    }

    fn f(addr: address) -> u32 {
        let _ = credits.aleo / Token { addr };
        return 0u32;
    }

    @noupgrade
    constructor() {}
}
