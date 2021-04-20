function main(s: bool, a: i128, b: i128, c: i128) {
    const r = s ? a : b;

    console.assert(r == c);
}