function main(s: bool, a: i16, b: i16, c: i16) {
    let r = s ? a : b;

    console.assert(r == c);
}