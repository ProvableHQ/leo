/*
namespace: Compile
expectation: Pass
input_file: input/fib.in
*/

function fib_tco_helper (const n: u32, sum: i64, prev: i64) -> i64  {
    if n <= 1 {
        return sum;
    } else {
        return fib_tco_helper(n-1, prev + sum, sum);
    }
}

function fib_tco(const n: u32) -> i64 {
    return fib_tco_helper(n, 0, 1);
}

function main() -> bool {
    let fib_tco = fib_tco(5);
    return fib_tco == 3;
}