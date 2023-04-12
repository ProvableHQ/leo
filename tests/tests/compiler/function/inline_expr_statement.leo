/*
namespace: Compile
expectation: Pass
configs:
    - dce_enabled : true
    - dce_enabled : false
*/

program test.aleo {

    inline function_that_returns_a_value() -> u32 {
        return 0u32;
    }

    inline another_function_that_returns_a_value(a: u32) -> u32 {
        return a * a;
    }

    transition some_function() {
        function_that_returns_a_value();
        let a: u32 = function_that_returns_a_value();
        another_function_that_returns_a_value(a);
        let b: u32 = another_function_that_returns_a_value(a);
    }
}