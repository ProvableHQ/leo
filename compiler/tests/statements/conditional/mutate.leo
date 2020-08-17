function main(a: u32) {
    let mut b = 5u32;

    if a == 1 {
        b = 1;
    } else {
        b = 0;
    }

    if a == 1 {
        console.assert(b == 1);
    } else {
        console.assert(b == 0);
    }
}
