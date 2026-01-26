program test.aleo {
    mapping foo: u32 => u32;
    fn main(x: u32, y: u32) -> Final {
        return final {
            let a = foo.get(x);
            foo.set(x, y);
        };
    }
}
