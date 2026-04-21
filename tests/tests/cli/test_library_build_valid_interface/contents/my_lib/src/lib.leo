// A well-formed interface should be accepted by the frontend when the
// library is built directly.
interface Counter {
    fn increment(amount: u64) -> u64;
}

fn one() -> u64 {
    return 1u64;
}
