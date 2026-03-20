program test.aleo {
    fn foo() -> (
        u32,
        Final<Fn(
            Final<Fn(
                Final<Fn(u32, Final<Fn(u32, u32)>, Final<Fn(u32, u32)>, u32)>,
                Final<Fn(u32, Final<Fn(u32, u32)>, Final<Fn(u32, u32)>, u32)>,
                Final<Fn(u32, Final<Fn(u32, u32)>, Final<Fn(u32, u32)>, u32)>,
            )>,
        )>,
    ) {
        let f: Final = final {};
        return (0u32, f);
    }
}
