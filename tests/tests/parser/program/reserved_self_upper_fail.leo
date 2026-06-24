program test.aleo {
    fn path_use() -> u32 {
        return Self::ctx::caller();
    }
    fn binding_use(Self: u32) -> u32 {
        return Self;
    }
    struct Self {
        x: u32,
    }
}
