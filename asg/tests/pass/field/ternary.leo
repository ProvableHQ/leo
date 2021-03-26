function main(s: bool, a: field, b: field, c: field) {
    const r = s ? a : b;

    console.assert(r == c);
}