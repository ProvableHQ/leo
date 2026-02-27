program test.aleo {
    fn main() {
        let (a, b) = (1u64, 2u64);
        let (x, y) = get_pair();
        let (p, q): (u64, u64) = (3u64, 4u64);
    }
}

fn get_pair() -> (u64, u64) {
    return (1u64, 2u64);
}
