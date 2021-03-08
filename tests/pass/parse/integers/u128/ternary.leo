function main(s: bool, a: u128, b: u128, c: u128) {
    let r = s ? a : b;

    console.assert(r == c);
}