
program test.aleo {
    async fn first(x: Future) -> Future {
        return finish();
    }

    fn second(x: Future) -> u8 {
        return 1u8;
    }
}

fn third(x: Future) -> u8 {
    return 1u8;
}

async fn finish() {
    assert_eq(1u8, 1u8);
}
