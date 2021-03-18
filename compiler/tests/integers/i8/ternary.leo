function main(s: bool, a: i8, b: i8, c: i8) {
    const r = s ? a : b;

    console.assert(r == c);
}