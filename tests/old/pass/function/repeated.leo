function one() -> bool {
    return true;
}

function main() {
    const a = one() && one();

    console.assert(a == true);
}