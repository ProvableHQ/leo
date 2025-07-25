program test.aleo {
    async transition foo(private a: u8) -> Future {
        return async {
            assert(a == 5);
        };
    }
}
