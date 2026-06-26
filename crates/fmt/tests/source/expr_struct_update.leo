struct Point  {
  x  :u64  ,
  y  : u64 ,
            z  :  u64  }

program test.aleo   {
    @noupgrade constructor() {}

    fn main(  base  :  Point  )  ->  Point   {
        let p  :  Point  =  Point  {   x  :  1u64  ,   ..base   }  ;
        let q  :  Point  =  Point  {   ..base   }  ;
        return p  ;
    }
}
