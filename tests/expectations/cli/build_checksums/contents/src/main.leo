// The 'build_checksums' program. Exercises `leo build --checksums`, which prints the program
// checksum and the checksum of each entry and view function (the `std::prog::function_checksum`
// targets).
program build_checksums.aleo {
    mapping vals: u32 => u32;

    view fn peek(k: u32) -> u32 {
        return vals.get_or_use(k, 0u32);
    }

    fn main(public a: u32, b: u32) -> u32 {
        return a + b;
    }

    @noupgrade
    constructor() {}
}
