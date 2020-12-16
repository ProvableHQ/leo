// A spread operator `...` copies the elements of one array into another
function main(a: [u8; 3]) {
    let b = [1u8, 1u8];
    let c = [1u8, ...b];

    console.assert(a == c);
}