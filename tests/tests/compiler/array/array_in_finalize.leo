
program test.aleo {
    async fn foo(a: [bool; 8]) -> Future {
        return fin_foo(a);
    }
}

async fn fin_foo(a: [bool; 8]) {
    assert_eq(true, true);
}
