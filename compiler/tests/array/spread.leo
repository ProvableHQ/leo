// A spread operator `...` copies the elements of one array into another
function main() -> u32[3] {
    let a = [1u32, 1u32];

    return [1u32, ...a]
}