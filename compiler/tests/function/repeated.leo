function one() -> bool {
    return true
}

function main() {
    let a = one() && one();

    assert_eq!(a, true);
}