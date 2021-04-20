function array_3x2_nested() -> [[u8; 2]; 3] {
    return [[0u8; 2]; 3];
}

function array_3x2_tuple() -> [[u8; 2]; 3] {
    return [0u8; (3, 2)];
}

function main() {
    const a = array_3x2_nested();
    const b = array_3x2_tuple();
}