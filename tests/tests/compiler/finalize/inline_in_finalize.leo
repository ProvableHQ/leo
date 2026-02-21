
program test.aleo {
    fn public_adder(public a: u8, public b: u8) -> Final {
        return final { finalize_public_adder(a, b); };
    }
}

fn foo(a: u8, b: u8) -> u8 {
    assert_neq(a, b);
    return a + b;
}

final fn finalize_public_adder(a: u8, b: u8) {
    let result: u8 = foo(a, b);
    assert_neq(result, 0u8);
}
