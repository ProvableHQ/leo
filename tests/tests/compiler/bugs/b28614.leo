program test.aleo {
    transition bar() {
        let x: u32 = self.caller;
        let y: u8 = self.caller;
        
        let z: u8 = 0u8;
        z = self.caller;

        let w: field = 0field;
        w = self.signer;
    }
}
