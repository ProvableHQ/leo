program test.aleo {
    fn foo() -> Final {
        let f = final {
            return 0u32;
        };
        return f;
    }

    fn foo2() -> Final {
        let f = final {
            return;
        };
        return f;
    }
}
