// Adding the `mut` keyword makes an array variable mutable.
function main() {
    let mut a = [1u32];
    a[0] = 0;

    assert_eq!(a[0], 0u32);
}