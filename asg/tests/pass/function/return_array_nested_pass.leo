function array_3x2_nested() -> [[u8; 2]; 3] {
    return [[0u8; 2]; 3];
}

function array_3x2_tuple() -> [[u8; 2]; 3] {
    return [0u8; (3, 2)];
}

function main() {
    let a = array_3x2_nested();
    let b = array_3x2_tuple();
}