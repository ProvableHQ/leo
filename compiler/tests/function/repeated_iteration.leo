function iteration() -> u32 {
    let mut a = 0u32;

    for i in 0..10 {
        a += 1;
    }

    return a
}

function main() -> u32 {
    return iteration() + iteration()
}