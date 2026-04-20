program test.aleo {
    mapping values: u32 => u32;

    fn main(public a: u32) -> Final {
        return final {
            assert_eq(fin_foo(a), fin_foo(a));
        };
    }
}

final fn fin_foo(a: u32) {
    Mapping::set(values, a, a);
}
