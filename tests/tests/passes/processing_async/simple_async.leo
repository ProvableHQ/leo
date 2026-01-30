program test.aleo {
    mapping foo: u32 => u32;
    fn main(x: u32, y: u32) -> Final {
        return final {
            foo.set(x, y);
        };
    }
}
