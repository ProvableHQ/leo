function main() {
    let x = 4u32;
    for i in 0..3 {
        x -= 1;
    }

    console.assert(x == 1u32);
}