// Import the circuits and functions from `imports/point.leo`.
// In testnet3, Leo imports all symbols from a file by default.
import point.leo;

// The main function.
@program
function main(public a: u32, b: u32) -> u32 {
    return a + b;
}
