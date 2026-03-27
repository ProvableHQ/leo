struct Foo{ // struct level note
    x:u8?, // optional field
    y:[u8;2], // array field
}

fn build_foo()->Foo{
    return Foo{ x:none, y:[1u8,2u8] };
}

program test.aleo{
    fn main()->[u8;2]{
        return build_foo().y;
    }
}
