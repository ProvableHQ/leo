program test.aleo {
    struct Foo {
        a: u128,
        b: u128,
    }

    transition main() {
        let addr_value: address = aleo10qerras5799u6k7rjtc9y3hcwxuykr45qra7x7dp6jgnc0923czqm0lgta;
        let bool_value: bool = true;
        let field_value: field = 1field;
        let group_value: group = group::GEN;
        let i128_value: i128 = 1i128;
        let u128_value: u128 = 1u128;
        let scalar_value: scalar = 1scalar;
        // let string_value: string = "helloworld";

        // BHP256 hash_to_scalar tests
        let a: scalar = ECDSAVerify::hash_to_scalar(addr_value);
        
        return (a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, q);
 
