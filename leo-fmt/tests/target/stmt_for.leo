program test.aleo {
    function main() -> u64 {
        let sum: u64 = 0u64;
        for i in 0u64..10u64 {
            sum += i;
        }
        for j: u64 in 0u64..5u64 {
            sum += j;
        }
        return sum;
    }
}
