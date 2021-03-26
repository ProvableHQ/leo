function main() {
    const a = (true, false);
    const b: (bool, (bool, bool)) = (true, a);
}