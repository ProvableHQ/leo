// Adding the `mut` keyword makes an array variable mutable.
function main() -> u32 {
    let mut a = [1u32];
    a[0] = 0;

    return a[0]
}