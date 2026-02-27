program test.aleo{
	fn main(  a
	        :  u64  )  ->  u64   {
    let result  :  u64  =  0u64  ;
            if  a  >  5u64  {  result  =
                1u64  ;  }
    if a  <  3u64
	{
                    result  =
                        2u64  ;
        }
if a  ==  4u64  {  result  = 3u64  ;  }  else  {  result  =
        4u64  ;  }
	    if a  >10u64{
            result  = 5u64
                ;
    }  else
                if a  >  7u64
	{  result  =
	    6u64  ;  }
            else  {
    result  = 7u64
        ;
        }
    return  result  ;
    }
}
