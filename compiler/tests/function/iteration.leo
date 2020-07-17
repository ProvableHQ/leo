function one() -> u32 {
    return 1u32
}

function main() -> u32 {
    let mut a = 0u32;

    for i in 0..10 {
        a += one();
    }

    return a
}