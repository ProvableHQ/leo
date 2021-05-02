// `{from}..{to}` copies the elements of one array into another exclusively
function main(a: [u8; 3]) {
    const b = [1u8; 4];

    console.assert(a == b[0u8..3i8]);
}
