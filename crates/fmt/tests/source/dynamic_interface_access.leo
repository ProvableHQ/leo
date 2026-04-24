interface Logger   {
    mapping  balances  :  address  =>  u64  ;
    storage  counter  :  u64  ;
    storage  entries  :  [  u64  ]  ;
    fn recompute(  public  x  :  u32  )  ->  public  u32  ;
    fn ping(  )  ->  public  u32  ;
}

program test.aleo   {
    fn query(  target  :  field  ,  key  :  address  ,  i  :  u32  )  ->  Final   {
        let  x  :  u32  =  Logger@(  target  )  ::  recompute(  1u32  )  ;
        let  y  :  u32  =  Logger@(  target  ,  'aleo'  )  ::  recompute(  1u32  )  ;
        let  z  :  u32  =  Logger@(  target  )  ::  ping(  )  ;
        return  final  {  finalize_query(  target  ,  key  ,  i  )  ;  }  ;
    }
}

final fn finalize_query(  target  :  field  ,  key  :  address  ,  i  :  u32  )   {
    let  a  :  u64   =   Logger@(  target  )  ::  balances  .  get(  key  )  ;
    let  b  :  bool  =  Logger@(  target  )  ::  balances  .  contains(  key  )  ;
    let  c  :  u64  =  Logger@(  target  )  ::  balances  .  get_or_use(  key  ,  0u64  )  ;
    let  d  :  u64  =  Logger@(  target  ,  'aleo'  )  ::  balances  .  get(  key  )  ;
    let  e  :  u64?  =  Logger@(  target  )  ::  counter  ;
    let  f  :  u64?  =  Logger@(  target  ,  'aleo'  )  ::  counter  ;
    let  g  :  u64?  =  Logger@(  target  )  ::  entries  .  get(  i  )  ;
    let  h  :  u32  =  Logger@(  target  )  ::  entries  .  len(  )  ;
    let  j  :  u32  =  Logger@(  target  ,  'aleo'  )  ::  entries  .  len(  )  ;
}
