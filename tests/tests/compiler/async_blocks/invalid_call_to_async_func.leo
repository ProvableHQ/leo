program test.aleo {
    async fn foo(private a: u8) -> Future {
        return async {
            bar();
        };
    }
}

async fn bar() {
}
