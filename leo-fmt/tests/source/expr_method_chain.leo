    struct Wrapper   {
                inner  :  u64,
	}

program test.aleo
  {
	mapping balances  :  address
	    =>  u64;

  fn deposit(
            public sender
                :  address  ,
    public amount
        :  u64  )  ->  Final  {
	    let bal  :  u64  =  balances.get(
                            sender
                                );
        let check  :  bool  =  balances.contains(
    sender  )
        ;
            balances.set(  sender  ,
	    amount  )
	        ;
balances.remove(
                sender  )  ;
    let val  :  u64  =  balances.get_or_use(  sender  ,
	        0u64  )
	            ;
            return finalize_deposit(
    sender  ,
        amount  )
            ;
        }

	fn use_special(
	        )  ->  address  {
let caller  :  address  =  self.caller
    ;
                let signer  :  address
    =  self.signer;
	    let h  :  u32  =  block.height
	        ;
            let t  :  u128  =
        block.timestamp
            ;
    let n  :  u16  =  network.id;
	                return
	                    caller  ;
}

      fn access_chain(
            w  :  Wrapper  ,
	    arr  :  [u64;  3]  ,
    tup  :  (u64,  u64)
        )  ->  u64   {
        let a  :  u64  =  w.inner
            ;
let b  :  u64
                =  arr[  0u32  ]  ;
	    let c  :  u64  =  tup.0
	        ;
            let d  :  u64  =  tup.1;
    return
        a  ;
  }
}
