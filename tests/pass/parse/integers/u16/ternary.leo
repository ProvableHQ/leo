function main(s: bool, a: u16, b: u16, c: u16) {
    const r = s ? a : b;

    console.assert(r == c);
}