program test.aleo {
    async transition foo() -> Future {
        let f = async {
            return 0u32;
        };
        return f;
    }

    async transition foo2() -> Future {
        let f = async {
            return;
        };
        return f;
    }
}
