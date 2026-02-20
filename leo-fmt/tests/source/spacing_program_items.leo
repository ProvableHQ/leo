program test.aleo
  {
struct A{
x:u64,
}
struct B{
  y :  u64  ,
    }



function foo( a  :  u64 )  ->u64 {
  return a;
    }




            function bar(b  :u64)  -> u64{
    return    b  ;
}
const VALUE :  u64=42u64;
        mapping store  :  u64  =>  u64;
  transition baz(  c :u64  )->  u64
{
            return c  ;
    }
}
