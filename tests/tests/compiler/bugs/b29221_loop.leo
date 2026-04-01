// Regression test for https://github.com/ProvableHQ/leo/issues/29221
// Using a const N as both the loop bound and the array length in a Deserialize
// type parameter verifies that loop unrolling and const propagation cooperate
// correctly when N appears in an intrinsic type parameter.

program test.aleo {
    const N: u32 = 2u32;

    fn main(bits: [bool; 182]) -> u32 {
        let arr: [u32; N] = Deserialize::from_bits::[[u32; N]](bits);
        let total: u32 = 0u32;
        for i: u32 in 0u32..N {
            total += arr[i];
        }
        return total;
    }
}
