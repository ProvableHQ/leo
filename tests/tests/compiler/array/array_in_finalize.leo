
program test.aleo {
    fn foo(a: [bool; 8]) -> Final {
        return fin_foo(a);
    }
}

onchain fn fin_foo(a: [bool; 8]) {
    assert_eq(true, true);
}
