	final fn do_mapping(
            addr  :  address  ,
            item
                :  u64
    )  {
    Mapping::set(  balances  ,
                    addr  ,
                        item  )
                            ;
	    Mapping::get(  balances  ,
        addr  )
            ;
    }

program test.aleo
    {
    mapping  balances  :  address  =>  u64  ;

	fn main(
            public item
                :  u64
    )  {
    }
}
