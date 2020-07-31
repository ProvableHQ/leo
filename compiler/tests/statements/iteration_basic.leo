function main() {
    let mut x = 4u32;
    for i in 0..3 {
        x -= 1;
    }

    assert_eq!(x, 1u32);
}