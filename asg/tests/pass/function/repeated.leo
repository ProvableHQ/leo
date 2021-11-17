function one() -> bool {
    return true;
}

function main() {
    let a = one() && one();

    console.assert(a == true);
}