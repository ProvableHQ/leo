program test.aleo {
    fn main(target: field) -> u32 {
        return Nonexistent@(target)/foo(1u32);
    }
}
