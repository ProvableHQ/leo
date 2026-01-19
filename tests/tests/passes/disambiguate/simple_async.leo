program test.aleo {
    mapping foo: u32 => u32;
    async fn main(x: u32, y: u32) -> Future {
        return async {
            let a = foo.get(x);
            foo.set(x, y);
        };
    }
}
