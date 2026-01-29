const a: u32 = 10u32;

program test.aleo {
    fn main(x: u32) -> u32 {
        let b: u32 = a + 5u32;
        return x + b;
    }
    fn foo(x: u32) -> u32 {
        let b: u32 = a + 5u32;
        if true {
            b = 42;
        }
        if false {
            b = 50;
        } else {
            b = 43;
        }
        if true && false{
            b = 60;
        }
        return x + b;
    }
}

