/*
namespece: Compile
expectation: Pass
*/

program test.aleo {

    inline function_that_returns_a_value() -> u32 {
        return 0u32;
    }

    transition some_function() {
        function_that_returns_a_value();
    }
}