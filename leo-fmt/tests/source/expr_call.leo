program test.aleo   {
	function add(   a  :  u64  ,
	    b  :  u64  )  ->  u64  {  return   a  +  b  ;  }

    function main(
            )  ->  u64   {

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

let mapped
    :   u64=Mapping::get(
    balances
        ,
                self.caller
    )  ;

            return
                x  ;
    }

	mapping balances  :  address  =>  u64  ;
}
