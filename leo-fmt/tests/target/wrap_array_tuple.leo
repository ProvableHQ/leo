program test.aleo {
    function main() -> u64 {
        let short_arr: [u64; 3] = [1u64, 2u64, 3u64];
        let long_arr: [u64; 5] = [
            first_long_value,
            second_long_value,
            third_long_value,
            fourth_long_value,
            fifth_long_value,
        ];
        let repeat: [u64; 10] = [0u64; 10];
        let short_tup: (u64, u64) = (1u64, 2u64);
        let long_tup: (u64, u64, u64) = (
            first_long_variable_name,
            second_long_variable_name,
            third_long_variable_name,
        );
        let single: (u64) = (42u64,);
        return 0u64;
    }
}
