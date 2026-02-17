program test.aleo
  {
struct Data{
amount:u64,
}
    mapping store:field  =>Data;


  fn save(d:Data)  ->  Final{

return finalize_save(
    1field  ,  d  )  ;
}

    final fn finalize_save(  key  :  field  ,
        d  :  Data  )  {
            store.set(  key  ,
                d  )  ;
    }
}
