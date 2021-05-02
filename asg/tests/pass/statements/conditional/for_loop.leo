function main(a: bool) {
    let b = 0u32;

    if a {
        for i in 0..4 {
            b += i;
        }
    }

    const r: u32 = a ? 6 : 0;

    console.assert(r == b);
}
