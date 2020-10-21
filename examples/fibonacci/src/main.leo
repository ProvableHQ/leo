// Computes the nth number of fibonacci
function fibonacci(n: u32) -> u32 {
    let mut a = 0u32;
    let mut b = 1u32;

    if n == 0 {
        return a
    } else if n <= 2 { 
        return b
    } else {
        for i in 1..n {
            let c = a + b;
            a = b;
            b = c;
        }
        return b
    }
}

// The 'fibonacci' main function.
function main(a: u32) -> u32 {
    return fibonacci(a)
}