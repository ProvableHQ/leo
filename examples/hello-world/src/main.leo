circuit BHP256 {
    function commit() -> bool {
        return true;
    }
}

function main(a: u8) -> u8 {
    return a >> 1u32;
}