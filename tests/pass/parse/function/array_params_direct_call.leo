function do_nothing(arr: [u32; 2]) {}

function main() {
    let arr: [u32; 2] = [0; 2];

    do_nothing(arr);
    do_nothing([...arr]);
    do_nothing(arr[1u32..]);
}