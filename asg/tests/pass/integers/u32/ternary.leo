function main(s: bool, a: u32, b: u32, c: u32) {
    let r = s ? a : b;

    console.assert(r == c);
}