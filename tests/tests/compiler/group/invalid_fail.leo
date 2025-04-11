
program test.aleo {
    transition main() -> group {
        // This should fail - not a valid group element.
        return 1group;
    }
}
