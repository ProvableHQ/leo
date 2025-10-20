program test.aleo {
       transition main(cond: bool, a: u32, b: u32) -> u32 {
        if cond {
            return a;
        } else {
            return b;
        }
    } 
}