function main(s: bool, a: group, b: group, c: group) {
    let r = if s ? a : b;

    console.assert(r == c);
}