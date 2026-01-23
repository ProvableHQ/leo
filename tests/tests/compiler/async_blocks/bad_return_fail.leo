program test.aleo {
    fn foo() -> Final {
        let f = async {
            return 0u32;
        };
        return f;
    }

    fn foo2() -> Final {
        let f = async {
            return;
        };
        return f;
    }
}
