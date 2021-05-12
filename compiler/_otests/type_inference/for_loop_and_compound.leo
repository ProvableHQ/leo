function main() {
    let x = 10u16;
    for i in 0..3 {
        x -= 1;
    }
    console.assert(x == 7u16);
}