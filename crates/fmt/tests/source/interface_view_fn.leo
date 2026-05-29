interface IToken  {
        view fn balance_of(  account  :  address  )  ->  u128  ;
   view fn  allowance(  owner  :  address  ,  spender  :  address  )  ->  u128  ;
        fn  transfer_public(  public recipient  :  address  ,  public amount  :  u128  )  ->  Final  ;
}
        program tok.aleo  :  IToken  {
    mapping balances  :  address  =>  u128  ;
    mapping allowances  :  field  =>  u128  ;

    @noupgrade
    constructor() {}

    view fn balance_of(  account  :  address  )  ->  u128  {
                return  balances  .  get_or_use(  account  ,  0u128  )  ;
        }

    view fn  allowance(  owner  :  address  ,  spender  :  address  )  ->  u128  {
                return  0u128  ;
        }

    fn  transfer_public(  public recipient  :  address  ,  public amount  :  u128  )  ->  Final  {
        return  final  {  }  ;
        }
}
