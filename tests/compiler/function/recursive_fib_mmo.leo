/*
namespace: Compile
expectation: Pass
input_file: input/fib.in
*/

function fib_memo(const n: u32, memo: [i64; 1000]) -> i64 {
    if n == 1 {
        return 0; 
    } else if n == 2 {
        return 1;
    } else if memo[n] == 0 {
        let fib1 = fib_memo(n-1, memo);
        let fib2 = fib_memo(n-2, memo);
        memo[n] = fib1 + fib2;
        return memo[n];
    } else {
        return memo[n];
    }
}

function main() -> bool {
    let fib_memo = fib_memo(5, [0i64; 1000]);
    return fib_memo == 3;
}