// Swap two elements of an array.
function swap(mut a: [u32; 2], const i: u32, const j: u32) -> [u32; 2] {
    let t = a[i];
    a[i] = a[j];
    a[j] = t;
    return a
}

function main() {
    let mut arr: [u32; 2] = [0, 1];
    let expected: [u32; 2] = [1, 0];

    // Do swap.
    let actual = swap(arr, 0, 1);

    // Check result.
    for i in 0..2 {
        console.assert(expected[i] == actual[i]);
    }
}