function main(a: bool, b: bool, c: u32) {
    let d = 0u32;

    if a {
        d += 1;
        if b {
            d += 2;
        }
    }

    console.assert(d == c);
}