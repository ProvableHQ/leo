function fibonacci() -> u32 {
    let mut a = 0u32;
    let mut b = 1u32;

    for i in 0..10 {
      a = b;
      b = a + b;
    }

    return a
}

// The 'fibonacci' main function.
function main() -> u32 {
    return fibonacci()
}