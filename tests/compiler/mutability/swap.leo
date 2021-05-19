/*
namespace: Compile
expectation: Pass
input_file: input/dummy.in
*/

// Swap two elements of an array.
function swap(a: [u32; 2], const i: u32, const j: u32) -> [u32; 2] {
    let t = a[i];
    a[i] = a[j];
    a[j] = t;
    return a;
}

function main(arr: [u32; 2]) -> bool {
    const expected: [u32; 2] = [1, 0];
    let actual = swap(arr, 0, 1);
    
    // Do swap.
    return expected[0] == actual[0] && expected[1] == actual[1];
}
