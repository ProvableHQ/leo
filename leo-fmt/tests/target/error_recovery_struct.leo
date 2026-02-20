program test.aleo {
    struct Broken {
        x: u64,
        transition foo(  a : u64  )  ->  u64{
      return   a  ;
    }
}
