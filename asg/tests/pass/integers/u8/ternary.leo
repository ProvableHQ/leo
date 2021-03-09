function main(s: bool, a: u8, b: u8, c: u8) {
    let r = s ? a : b;

    console.assert(r == c);
}