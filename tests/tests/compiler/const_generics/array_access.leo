program test.aleo {
    fn main() -> u32 {
        let arr: [u32; 6] = [1, 2, 3, 4, 5, 6];
        let sum = 0u32;
        for i: u32 in 0..6 {
            sum += at_n::[i](arr);
        }
        return sum;
    }
}

fn at_n::[N: u32](arr: [u32; 6]) -> u32 {
    return arr[N];
}
