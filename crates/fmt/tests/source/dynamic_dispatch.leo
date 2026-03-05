program   test.aleo   {

    fn   transfer(  r  :   dyn   record   )  ->   dyn   record   {
        return   r   ;
    }

    fn   call_no_ret(  prog  :  field  ,  net  :  field  ,  func  :  field  ,  arg  :  u64  )  {
        _dynamic_call(  prog  ,  net  ,  func  ,  arg  )  ;
    }

    fn   call_u64(  prog  :  field  ,  net  :  field  ,  func  :  field  ,  arg  :  u64  )  ->  u64  {
        let   result  :  u64  =  _dynamic_call::[  public  u64  ](  prog  ,  net  ,  func  ,  arg  )  ;
        return  result  ;
    }

    fn   call_dyn(  prog  :  field  ,  net  :  field  ,  func  :  field  ,  r  :  dyn   record  )  ->  dyn   record  {
        let   out  :  dyn   record  =  _dynamic_call::[  dyn   record  ](  prog  ,  net  ,  func  ,  r  )  ;
        return   out  ;
    }

    mapping   balances  :  address  =>  u64  ;

    fn   check(  prog  :  field  ,  net  :  field  ,  m  :  field  ,  key  :  address  )  ->  Final  {
        return   final   {   finalize_check(  prog  ,  net  ,  m  ,  key  )  ;   }   ;
    }
}

final   fn   finalize_check(  prog  :  field  ,  net  :  field  ,  m  :  field  ,  key  :  address  )  {
    let   exists  :  bool  =  _dynamic_contains(  prog  ,  net  ,  m  ,  key  )  ;
    let   val  :  u64  =  _dynamic_get::[  u64  ](  prog  ,  net  ,  m  ,  key  )  ;
    let   val2  :  u64  =  _dynamic_get_or_use::[  u64  ](  prog  ,  net  ,  m  ,  key  ,  0u64  )  ;
    assert(  exists  )  ;
}
