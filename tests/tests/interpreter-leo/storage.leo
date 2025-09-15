program test.aleo {
    storage counter: u32;
    storage vec: [u32];
    async transition main() -> Future {
        let f = async {
            counter = 3u32;
            counter = None;
            counter = 4u32;
            assert(counter == 4);

            vec.push(42);
            assert(vec.get(0).unwrap() == 42u32);
        };
        f.await();
        return f;
    }
}
