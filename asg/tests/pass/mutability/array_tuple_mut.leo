// Adding the `mut` keyword makes an array variable mutable.
function main() {
    let mut a = [(1u32, 2u32)];
    a[0u32].1 = 3u32;

    console.assert(a[0u32].0 == 1u32);
    console.assert(a[0u32].1 == 3u32);
}