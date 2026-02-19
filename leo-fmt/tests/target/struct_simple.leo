struct Point {
    x: u64,
    y: u64,
}

program test.aleo {
    fn main() -> Point {
        return Point { x: 1u64, y: 2u64 };
    }
}
