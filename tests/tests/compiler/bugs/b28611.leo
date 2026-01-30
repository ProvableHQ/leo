program test.aleo {
    mapping m: u8 => u8;

    fn foo(sig: signature, x: u32, y: u32) {
        // ensure that the caller provided the correct signature
        let t: (u32, u32) = (x, y);
        let b: bool = sig.verify(self.caller, t);
        let c: bool = sig.verify(self.caller, m);
    }
}
