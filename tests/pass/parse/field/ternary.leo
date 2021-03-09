function main(s: bool, a: field, b: field, c: field) {
    let r = s ? a : b;

    console.assert(r == c);
}