// Adding the `mut` keyword makes a variable mutable.
function main() {
    let a = 1u32;
    a = 0;

    console.assert(a == 0u32);
}