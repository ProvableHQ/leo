program test.aleo
    {
	transition main(
            item
                :  u64
    )  {
    Mapping::set(  balances  ,
                    self.caller  ,
                        item  )
                            ;
	    Mapping::get(  balances  ,
        self.caller  )
            ;
    }
}
