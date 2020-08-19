function main(s: bool, a: u16, b: u16, c: u16) {
    let r = if s ? a : b;

    console.assert(r == c);
}