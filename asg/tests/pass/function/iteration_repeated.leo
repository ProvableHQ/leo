function iteration() -> u32 {
    let a = 0u32;

    for i in 0..10 {
        a += 1;
    }

    return a;
}

function main() {
    const total = iteration() + iteration();

    console.assert(total == 20);
}