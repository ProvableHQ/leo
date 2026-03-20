struct Foo{
    x:u8?,
}

struct Wrapper{
    arr:[Foo?;2],
}

program test.aleo{
    fn main()->Wrapper{
        return Wrapper{
            arr:[Foo{x:8u8},none], // implicit wrap
        };
    }
}
