/*
namespace: Compile
expectation: Pass
input_file: input/fib.in
*/

function fib(const n: u32) -> i64 {
    if n == 1 {
        return 0;
    } else if n == 2 {
        return 1;
    } else {
        return fib(n-1) + fib(n-2);
    }
}

function main(a: u32) -> bool {
    let fib = fib(5);
    return fib == 3;
}