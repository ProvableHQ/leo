program test.aleo
            {
	function with_expr(  a  :  u64  )  ->  u64  {
    return   a  *
                    2u64
                        ;
        }

	function bare_return(
	        )  ->  (  )  {
            return
                ;
    }

        function with_tuple(  a  :  u64  ,
	    b
	        :  u64  )  ->  (  u64  ,  u64  )  {
return    (  a  +  b  ,
            a  *  b  )
                ;
    }
}
