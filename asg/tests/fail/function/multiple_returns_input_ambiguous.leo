function main(input) -> u32 {
    if input.registers.a == 0 {
        return 0u32;
    } else {
        return 1u32;
    }
}