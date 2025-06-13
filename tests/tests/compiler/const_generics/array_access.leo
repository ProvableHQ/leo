program test.aleo {
    transition main() -> u32 {
        let arr: [u32; 6] = [1, 2, 3, 4, 5, 6];
        let sum = 0u32;
        for i: u32 in 0..6 {
            sum += at_n::[i](arr);
        }
        return sum;
    }

    inline at_n::[N: u32](arr: [u32; 6]) -> u32 {
        return arr[N];
    }

    const M: u32 = 1;
    transition main2() -> u32 {
        let total = 0u32;
        total += sum_array::[M]([1]);
        total += sum_array::[2]([1, 2u32]);
        total += sum_array::[3]([1, 2u32, 3u32]);
        total += sum_array::[4]([1, 2u32, 3u32, 4u32]);

        return total; 
    }

    inline sum_array::[N: u32](arr: [u32; N]) -> u32 {
        let sum = 0u32;
        for i in 0u32..N {
            sum += arr[i];
        }
        return sum;
    }
}
