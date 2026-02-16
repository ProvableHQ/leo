program test.aleo {
    function short(a: u64, b: u64) -> u64 {
        return a + b;
    }

    function long_parameter_name_function(first_very_long_parameter_name: u64, second_very_long_parameter_name: u64, third_very_long_parameter_name: u64) -> u64 {
        return first_very_long_parameter_name + second_very_long_parameter_name;
    }

    transition process(public input_value: field, private secret_value: field) -> field {
        return input_value + secret_value;
    }

    transition very_long_transition_name(public first_parameter_name: field, private second_parameter_name: field, constant third_parameter_name: field) -> field {
        return first_parameter_name;
    }
}
