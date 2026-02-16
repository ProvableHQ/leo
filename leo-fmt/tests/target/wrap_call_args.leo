program test.aleo {
    function add(a: u64, b: u64) -> u64 {
        return a + b;
    }

    function main() -> u64 {
        let short: u64 = add(1u64, 2u64);
        let long_result: field = BHP256::hash_to_field(
            some_very_long_variable_name_that_pushes_over_the_limit + another_value,
        );
        let another: u64 = some_module::some_very_long_function_name(
            first_argument_value,
            second_argument_value,
            third_argument_value,
        );
        return short;
    }
}
