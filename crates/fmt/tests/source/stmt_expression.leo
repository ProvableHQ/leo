program test.aleo
    {
	fn main(
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
