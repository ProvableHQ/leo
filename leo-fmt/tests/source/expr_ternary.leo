program test.aleo
{
	fn main(  a  :  u64  ,
	        b
	            :  u64  )  ->  u64  {

let result  :u64  =  a  >  b
    ?  a
        :  b
        ;

            let nested:u64  =a  >b  ?  a  >  10u64
                ?  a  :
                    10u64  :  b;

	    return
	        result  ;

    }
}
