	fn compute_sum(  first  :  u64  ,  second  :u64  ,  third  :  u64  )  ->  u64  {
        return  first  +
            second  +  third
                ;  }

program test.aleo
    {

    fn main(   public   a  :  u64  ,
                public  b  :  u64  )  ->  u64  {

let short  :u64  =  compute_sum(
    a  ,
        b  ,
            a  )  ;
	    let first_value
	        :  u64  =  a  *  2u64  ;
            let second_value  :u64  =b  *
                3u64  ;
    let third_value  :  u64
        =a  +  b  ;
let long  :  u64  =  compute_sum(  first_value  +  second_value  ,  second_value  +  third_value  ,  third_value  +  first_value  )
    ;
                    return short  +
                        long  ;
    }
}
