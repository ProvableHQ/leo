function main(s: bool, a: group, b: group, c: group) {
    const r = s ? a : b;

    console.assert(r == c);
}