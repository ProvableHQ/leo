function main() {
    let s = "\u{2764}ello, World!\u{DDDD}";
    s[..2] = "he";
}
