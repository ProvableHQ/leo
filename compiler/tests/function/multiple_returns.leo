function tuple() -> (bool, bool) {
    return (true, false)
}

function main() {
    const (a, b) = tuple();

    console.assert(a == true);
    console.assert(b == false);
}