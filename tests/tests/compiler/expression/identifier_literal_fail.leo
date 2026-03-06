program test.aleo {
    fn too_long(x: identifier) -> bool {
        // 32-byte identifier literal exceeds the 31-byte maximum.
        return x == 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa';
    }
}
