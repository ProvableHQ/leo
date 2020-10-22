function main() -> u8 {
    let mut a = (1, 2);

    a.0 = 3u32;

    return a.1
}