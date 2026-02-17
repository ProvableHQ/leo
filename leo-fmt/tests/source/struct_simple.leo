program test.aleo{
struct Point{
  x :  u64,
        y:u64}

  transition main(  )  ->  Point  {
    return Point  {  x  :  1u64  ,  y  :  2u64  }  ;
    }
}
