program test.aleo {
    fn foo(private a: u8) -> Future {
        return async {
            assert(a == 5);
        };
    }
}
