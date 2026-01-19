program test.aleo {
    async fn foo() -> Future {
        let f = async {
            return 0u32;
        };
        return f;
    }

    async fn foo2() -> Future {
        let f = async {
            return;
        };
        return f;
    }
}
