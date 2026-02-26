
program test.aleo {
    fn first(x: Final) -> Final {
        return final { finish(); };
    }

    fn second(x: Final) -> u8 {
        return 1u8;
    }
}

fn third(x: Final) -> u8 {
    return 1u8;
}

final fn finish() {
    assert_eq(1u8, 1u8);
}
