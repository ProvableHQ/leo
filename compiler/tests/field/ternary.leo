function main(s: bool, a: field, b: field, c: field) {
    let r = if s ? a : b;

    console.assert(r == c);
}