function main(a: bool, b: bool) -> u32 {
    let mut result = 0u32;

    if a {
        result += 1;
        if b {
            result += 2;
        }
    }

    return result
}