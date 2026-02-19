	fn add(   a  :  u64  ,
	    b  :  u64  )  ->  u64  {  return   a  +  b  ;  }

    fn to_field(
	    a  :  u64  )  ->  field  {  return  a  as  field  ;  }

program test.aleo   {
    fn main(
            )  ->  u64   {

let x
    :u64  =add(
        1u64
            ,
            2u64  )  ;

                    let y:u64  =  add(
                        1u64  ,  2u64  )  ;


	    let hash
	        :field  =  to_field(
	                1u64  )  ;

            return
                x  ;
    }
}
