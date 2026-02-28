	fn add(   a  :  u64  ,
	    b  :  u64  )  ->  u64  {  return   a  +  b  ;  }

	final fn do_map(
	        addr  :  address  )  {
let mapped
    :   u64=Mapping::get(
    balances
        ,
                addr
    )  ;
	}

program test.aleo   {
    fn main(
            public  addr  :  address  )  ->  u64   {

let x
    :u64  =add(
        1u64
            ,
            2u64  )  ;

                    let y:u64  =  add(
                        1u64  ,  2u64  )  ;


	    let hash
	        :field  =  BHP256::hash_to_field(
	                1u64  )  ;

            return
                x  ;
    }

	mapping balances  :  address  =>  u64  ;
}
