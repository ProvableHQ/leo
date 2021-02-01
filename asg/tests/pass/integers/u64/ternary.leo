function main(s: bool, a: u64, b: u64, c: u64) {
    let r = if s ? a : b;

    console.assert(r == c);
}