program test.aleo {
    fn main(arr: [u32; 8]) {
        let s1 = arr[2..5];
        let s2 = arr[..4];
        let s3 = arr[3..];
        let s4 = arr[1..=6];
    }
}
