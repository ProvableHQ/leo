program test.aleo {
    fn main(x: u32) -> Final {
        let f = final {
            assert(x == 3);
        };
        let y0 = f.0;
        let z = f;
        let y1 = z.3;
        return f;
    }
}
