program test.aleo {
    fn foo(private a: u8) -> Final {
        return final {
            bar();
        };
    }
}

onchain fn bar() {
}
