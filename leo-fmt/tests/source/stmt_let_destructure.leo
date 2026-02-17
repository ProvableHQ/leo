program test.aleo
  {
	fn main(  )   {
            let (  a  ,
                b  )  =  (  1u64  ,
                    2u64  )  ;
    let (  x  ,
	    y  )  =  get_pair(
                    1u64  )
                        ;
let (  p  ,  q  )  :  (  u64  ,  u64  )  =  (
        3u64  ,
        4u64  )  ;
    }

	fn get_pair(
	        n  :  u64  )  ->  (  u64  ,  u64  )   {
            return (  n  ,
    n  )
        ;
    }
}
