program b28789.aleo {
    @custom
    constructor()  {
        // This should be an error and not a compiler panic.
        return true;
    }
    
    fn main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
}

