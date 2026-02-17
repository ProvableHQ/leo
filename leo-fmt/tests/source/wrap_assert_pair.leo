program test.aleo  {
    fn main(
	    public  a  :  u64  ,
	        public  b  :  u64  )  {

assert_eq(
    a  ,
            b  )  ;
    let expected_first_balance  :u64
        =  a  +  b  ;
            let computed_second_balance
                :  u64  =  a  *  2u64  ;
assert_eq(  expected_first_balance  +  computed_second_balance  ,  computed_second_balance  +  expected_first_balance  )  ;
	    assert_neq(  a  ,
	        0u64  )  ;
                assert_neq(  expected_first_balance  *  computed_second_balance  ,  computed_second_balance  *  expected_first_balance  )  ;
}  }
