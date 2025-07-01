
program test.aleo {

    inline foo(a: u8, b: u8) -> u8 {
        assert_neq(a, b);
        return a + b;
    }

    async transition public_adder(public a: u8, public b: u8) -> Future {
        return async {
            let result: u8 = foo(a, b);
            assert_neq(result, 0u8);
        };
    } 
}
