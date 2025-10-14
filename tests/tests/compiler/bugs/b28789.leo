program b28789.aleo {
    @custom
    async constructor()  {
        // This should be an error and not a compiler panic.
        return true;
    }
    
    transition main(public a: u32, b: u32) -> u32 {
        let c: u32 = a + b;
        return c;
    }
}

