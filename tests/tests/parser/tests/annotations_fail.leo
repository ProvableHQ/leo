
program test.aleo {
    @some_annotation
    function foo1() -> u8 {
        return 1u8;
    }

    @some_annotation(some_key = "SomeString")
    function foo2() -> u8 {
        return 2u8;
    }

    @some_annotation(some_key = "SomeString", another_key = "AnotherString",)
    function foo3() -> u8 {
        return 3u8;
    }

    @failing_annotation(some_key = "SomeString", another_key = "AnotherString" oops)
    function foo4() -> u8 {
        return 4u8;
    }
}
