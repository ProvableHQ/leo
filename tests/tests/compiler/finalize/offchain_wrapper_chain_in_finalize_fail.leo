// FAIL: transitivity through the call graph — `deep` calls `mid` calls `who` calls
// `std::ctx::caller()`. Invoking `deep()` from a `final {}` block must still be rejected,
// even though `deep` never directly touches an off-chain intrinsic itself.
program test.aleo {
    fn main() -> Final {
        return final { let _ = deep(); };
    }

    @noupgrade
    constructor() {}
}

export fn who() -> address {
    return std::ctx::caller();
}

export fn mid() -> address {
    return who();
}

export fn deep() -> address {
    return mid();
}
