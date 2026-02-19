    fn process(  a  :  u64  ,
                    b  :  u64  ,
                        c  :  u64  )
                            {
    assert(  a  >
        0u64  )  ;  }

program test.aleo
    {
	fn main(
            item
                :  u64
    )  {
    process(  item  ,
                    item  ,
                        item  )
                            ;
	    process(  item  ,
        1u64  ,
            2u64  )
            ;
    }
}
