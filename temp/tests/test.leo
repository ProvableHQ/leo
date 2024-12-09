// A Leo test file.
// To learn more about testing your program, see the documentation at https://docs.leo-lang.org
import helloworld.aleo;

program test.aleo {
    @compiled_test
    @interpreted_test
    transition test_helloworld() {{
        let result: u32 = helloworld.aleo/main(1u32, 2u32);
        assert_eq(result, 3u32);
    }}
}

