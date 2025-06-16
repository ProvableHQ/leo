program test.aleo {
    transition main() -> bool {
        return true;
    }

    function test_reserved() {
        let constructor: u32 = 42u32;
    }
}
