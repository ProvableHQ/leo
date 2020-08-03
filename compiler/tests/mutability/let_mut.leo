// Adding the `mut` keyword makes a variable mutable.
function main() {
    let mut a = 1u32;
    a = 0;

    assert_eq!(a, 0u32);
}