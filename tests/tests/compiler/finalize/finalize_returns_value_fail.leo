
program test.aleo {
    fn public_adder(public a: u8, public b: u8) -> Final {
        return final { finalize_public_adder(a, b); };
    }
}

final fn finalize_public_adder(a: u8, b: u8) -> public u8 {
    return a + b;
}
