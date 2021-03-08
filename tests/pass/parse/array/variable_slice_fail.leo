function main() {
    let a = [1u8; 10];
    for i in 0..10 {
        let x = a[i..10];
        console.debug("{}", x);
    }
}