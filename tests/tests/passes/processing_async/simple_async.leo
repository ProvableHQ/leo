program test.aleo {
    mapping foo: u32 => u32;
    async transition main(x: u32, y: u32) -> Future {
        return async {
            foo.set(x, y);
        };
    }
}
