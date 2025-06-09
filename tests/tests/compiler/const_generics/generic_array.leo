program test.aleo {
    transition main() -> u32 {
        let total = 0u32;
        total += sum_array::[1]([1u32]);
        total += sum_array::[2]([1u32, 2u32]);
        total += sum_array::[3]([1u32, 2u32, 3u32]);
        total += sum_array::[4]([1u32, 2u32, 3u32, 4u32]);

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
