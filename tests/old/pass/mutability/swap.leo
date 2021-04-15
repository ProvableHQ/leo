// Swap two elements of an array.
function swap(a: [u32; 2], const i: u32, const j: u32) -> [u32; 2] {
    const t = a[i];
    a[i] = a[j];
    a[j] = t;
    return a;
}

function main() {
    let arr: [u32; 2] = [0, 1];
    const expected: [u32; 2] = [1, 0];

    // Do swap.
    const actual = swap(arr, 0, 1);

    // Check result.
    for i in 0..2 {
        console.assert(expected[i] == actual[i]);
    }
}