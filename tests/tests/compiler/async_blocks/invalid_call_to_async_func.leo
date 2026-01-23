program test.aleo {
    fn foo(private a: u8) -> Final {
        return async {
            bar();
        };
    }
}

onchain fn bar() {
}
