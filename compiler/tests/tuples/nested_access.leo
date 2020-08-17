function main() {
    let a = (true, false);
    let b = (true, a);

    console.assert(b.0 == true);
    console.assert(b.1.0 == true);
    console.assert(b.1.1 == false);
}