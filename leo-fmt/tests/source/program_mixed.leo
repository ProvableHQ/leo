program test.aleo
  {
struct Data{
amount:u64,
}
    mapping store:field  =>Data;


  async transition save(d:Data)  ->  Future{

return finalize_save(
    1field  ,  d  )  ;
}

    async function finalize_save(  key  :  field  ,
        d  :  Data  )  {
            store.set(  key  ,
                d  )  ;
    }
}
