program test.aleo {
    async transition main(x: u32) -> Future {
        let f = async {
            assert(x == 3);
        };
        let y0 = f.0;
        let z = f;
        let y1 = z.3;
        return f;
    }
}
