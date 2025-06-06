program test.aleo {
    const N1: u8 = 0;
    const N2: u16 = 1;
    const N3: u32 = 2;
    const N4: u8 = 3u8;
    const N5: u16 = 4u16;
    const N6: u32 = 5u32;
    transition main() -> u32 {
        let arr: [u32; 6] = [1, 2, 3, 4, 5, 6];
        return arr[N1] + arr[N2] + arr[N3] + arr[N4] + arr[N5] + arr[N6];
    }
}
