function main() {
    let a = (true, false);

    console.assert(a.0 == true);
    console.assert(a.1 == false);
}