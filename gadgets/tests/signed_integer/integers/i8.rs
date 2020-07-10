use leo_gadgets::signed_integer::Int8;

#[test]
fn test_i8() {
    let i8 = Int8::constant(-1i8);

    println!("{:?}", i8.value);
    println!("{:?}", i8.bits);
}

#[test]
fn test_constant() {
    test_constant!(i8, Int8);
}
