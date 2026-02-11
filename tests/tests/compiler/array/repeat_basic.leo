const N: u32 = 2u32;
const M: u32 = 1u32;

program test.aleo {
    fn main() -> u32 {
        let arr: [u32; 10u8] = [3u32; 10u8];
        let arr1: [u32; 10u8] = [3u32; 10];
        let arr2 = [3u32; 10];
        let arr3: [u8; N + M] = [1, 2, 3];
        arr3 = [0; N + M];
        return arr3[0] as u32 + [42u32; 3][2];
    }
}
