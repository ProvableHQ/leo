program test.aleo{
  mapping   balances :  address   =>u64
;

    fn deposit(  amount  :  u64  )  ->  Final  {
        let caller  :  address  =  self.caller  ;
	    return final  {  finalize_deposit(
	        caller  ,  amount  )  ;  }
    }
}

  final fn finalize_deposit(  caller  :  address  ,
        amount  :  u64  )  {
            balances.set(  caller  ,
                amount  )  ;  }
