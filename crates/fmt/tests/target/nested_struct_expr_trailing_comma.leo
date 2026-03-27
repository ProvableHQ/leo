struct Row {
    c1: u8,
    c2: u8,
    c3: u8,
}

struct Board {
    r1: Row,
    r2: Row,
    r3: Row,
}

program test.aleo {
    fn main() -> Board {
        return Board {
            r1: Row { c1: 0u8, c2: 0u8, c3: 0u8 },
            r2: Row { c1: 0u8, c2: 0u8, c3: 0u8 },
            r3: Row { c1: 0u8, c2: 0u8, c3: 0u8 },
        };
    }
}
