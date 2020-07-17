function tuple() -> (bool, bool) {
    return (true, false)
}

function main() -> (bool, bool) {
    let (a, b) = tuple();
    return (a, b)
}