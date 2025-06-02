program foo.aleo {
    transition bad(x: u32, y: u32, z: u32) {
        // All of these definitions are bad
        let t0 = (x, (y, z));
        let t1: u32 = (x, y);
        let t2: (u32, u32) = (x, y, z);
        let t3: (u32, (u32, u32)) = (x, (y, z));
        let t4: (u32, u8) = (x, y);
        let (a, b) = (x, y, z);
        let (a, b): u32 = (x, y);
        let (a, b): (u32, u8) = (x, y);
    }
}
