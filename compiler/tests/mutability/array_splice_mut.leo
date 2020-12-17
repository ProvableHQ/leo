// Adding the `mut` keyword makes an array variable mutable.
function main() {
    let mut a = [1u32, 2u32, 3u32];
    a[0u32..2u32] = [4u32, 5u32];

    console.assert(a[0] == 4u32);
    console.assert(a[1] == 5u32);
    console.assert(a[2] == 3u32);
}