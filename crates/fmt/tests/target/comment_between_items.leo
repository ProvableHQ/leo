// File-level comment about this program
// A struct
struct Point {
    x: u64,
    y: u64,
}
// A function that uses the struct
fn make_point(x: u64, y: u64) -> Point {
    return Point { x: x, y: y }; // construct it
}
// Another function
// with a multi-line comment
fn add(a: u64, b: u64) -> u64 {
    let result: u64 = a + b; // do the add
    // check it
    assert(result > 0u64);
    return result;
}
program test.aleo {
    // A transition too
    fn check(a: u64, b: u64) -> bool {
        let x: bool = a >= b;
        return x; // done
    }
}
