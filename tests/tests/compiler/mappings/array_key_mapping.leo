program test.aleo {
    mapping allowances: [address; 2] => u128;

    fn foo(key0: address, key1: address) -> Final {
        let key: [address; 2] = [key0, key1];
        return final { fin_foo(key); };
    }
}

final fn fin_foo(key: [address; 2]) {
    let val: u128 = allowances.get_or_use(key, 0u128);
}
