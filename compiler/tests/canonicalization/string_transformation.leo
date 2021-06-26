function main() {
    let s = "\u{2764}ello, World!\u{DDDD}";
    s[..2] = "he";
    let x = false;
    x = "test1" == "test2";
    let z = [1u8, 2u8, 3u8, 4u8];
    z[0.."test" == "test"? 2 : 2] = x[0..2];
}
