program test.aleo {
    mapping Yo: address => u32;

    fn main(a: address) {
        assert_eq(a, std::ctx::caller());
        assert_eq(a, std::ctx::addr());
        assert_eq(a, hello.aleo);
        return foo.aleo;
    }
}
