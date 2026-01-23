program test.aleo {
    fn foo() -> Future {
        let f = async {
            return 0u32;
        };
        return f;
    }

    fn foo2() -> Future {
        let f = async {
            return;
        };
        return f;
    }
}
