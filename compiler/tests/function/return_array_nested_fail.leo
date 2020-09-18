function array_3x2_tuple() -> [[u8; 2]; 3] {
    return [0u8; (2, 3)] // The correct 3x2 array tuple is `[0u8; (3, 2)]`
}

function main() {
    let b = array_3x2_tuple();
}