function main(s: bool, a: i8, b: i8, c: i8) {
    let r = if s ? a : b;

    console.assert(r == c);
}