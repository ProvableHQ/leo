program test.aleo {
    function main() {
        let arr: [u64; 3] = [1u64, 2u64, 3u64];
        let tup: (u64, u64, u64) = (1u64, 2u64, 3u64);
        let nested: [[u64; 2]; 2] = [[1u64, 2u64], [3u64, 4u64]];
        let parens: u64 = (1u64 + 2u64) * 3u64;
        let repeat: [u64; 4] = [0u64; 4u64];
    }
}
