
program test.aleo {
    async transition first(x: Future) -> Future {
        return finish();
    }

    transition second(x: Future) -> u8 {
        return 1u8;
    }

    function third(x: Future) -> u8 {
        return 1u8;
    }

    async function finish() {
        assert_eq(1u8, 1u8);
    }
}
