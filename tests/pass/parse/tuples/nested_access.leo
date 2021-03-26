function main() {
    const a = (true, false);
    const b = (true, a);

    console.assert(b.0 == true);
    console.assert(b.1.0 == true);
    console.assert(b.1.1 == false);
}