fn sum::[N: u32]() -> u32 {
    let s = 0u32;
    for i in 0u32..N {
        s += i;
    }
    return s;
}

fn use_generics() {
    let x = Array::[u32, 32u32] { data: [0u32; 32u32] };
    let y = Matrix::<ROWS, 4>;
}
