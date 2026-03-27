program test.aleo{
    fn main(public a:u32)->Final<Fn(u32)>{
        return final { finalize_value(a); };
    }
}

final fn finalize_value(a:u32){
    assert_eq(a, a);
}

final fn finalize_nested(f:Final<Fn(Final<Fn(u32,u32)>,Final<Fn(u32,u32)>,Final<Fn(u32,u32)>)>){
    f.run();
}
